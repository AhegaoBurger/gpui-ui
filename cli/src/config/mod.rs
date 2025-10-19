use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub const CONFIG_FILE_NAME: &str = "gpui-ui.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_component_path")]
    pub component_path: String,

    #[serde(default = "default_utils_path")]
    pub utils_path: String,

    #[serde(default = "default_gpui_version")]
    pub gpui_version: String,

    #[serde(default)]
    pub style: StyleConfig,

    #[serde(default)]
    pub components: Vec<InstalledComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StyleConfig {
    #[serde(default)]
    pub colors: ColorConfig,

    #[serde(default = "default_radius")]
    pub radius: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorConfig {
    #[serde(default = "default_primary")]
    pub primary: String,

    #[serde(default = "default_secondary")]
    pub secondary: String,

    #[serde(default = "default_destructive")]
    pub destructive: String,

    #[serde(default = "default_muted")]
    pub muted: String,

    #[serde(default = "default_accent")]
    pub accent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledComponent {
    pub name: String,
    pub version: String,
    pub installed_at: String,
}

// Default functions
fn default_component_path() -> String {
    "src/components/ui".to_string()
}

fn default_utils_path() -> String {
    "src/lib".to_string()
}

fn default_gpui_version() -> String {
    "0.2.1".to_string()
}

fn default_radius() -> String {
    "px(4.0)".to_string()
}

fn default_primary() -> String {
    "rgb(0x3b82f6)".to_string()
}

fn default_secondary() -> String {
    "rgb(0x64748b)".to_string()
}

fn default_destructive() -> String {
    "rgb(0xef4444)".to_string()
}

fn default_muted() -> String {
    "rgb(0xf1f5f9)".to_string()
}

fn default_accent() -> String {
    "rgb(0xf0f9ff)".to_string()
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            primary: default_primary(),
            secondary: default_secondary(),
            destructive: default_destructive(),
            muted: default_muted(),
            accent: default_accent(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            component_path: default_component_path(),
            utils_path: default_utils_path(),
            gpui_version: default_gpui_version(),
            style: StyleConfig::default(),
            components: Vec::new(),
        }
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        let content = fs::read_to_string(&config_path)
            .context(format!("Failed to read config file at {:?}", config_path))?;
        let config: Config = serde_json::from_str(&content)
            .context("Failed to parse config file")?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;
        fs::write(&config_path, content)
            .context(format!("Failed to write config file to {:?}", config_path))?;
        Ok(())
    }

    pub fn exists() -> bool {
        Self::get_config_path()
            .map(|path| path.exists())
            .unwrap_or(false)
    }

    fn get_config_path() -> Result<PathBuf> {
        let current_dir = std::env::current_dir()
            .context("Failed to get current directory")?;
        Ok(current_dir.join(CONFIG_FILE_NAME))
    }

    pub fn add_component(&mut self, name: String, version: String) {
        let installed_at = chrono::Utc::now().to_rfc3339();
        self.components.push(InstalledComponent {
            name,
            version,
            installed_at,
        });
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
