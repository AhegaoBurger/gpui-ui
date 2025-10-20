use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub files: Vec<String>,
    pub dependencies: Vec<String>,
}

pub struct Registry {
    components: HashMap<String, ComponentInfo>,
}

impl Registry {
    pub fn new() -> Self {
        let mut components = HashMap::new();

        // Button component
        components.insert(
            "button".to_string(),
            ComponentInfo {
                name: "button".to_string(),
                description: "A customizable button component with multiple variants".to_string(),
                version: "0.1.0".to_string(),
                files: vec!["button.rs".to_string()],
                dependencies: vec!["traits".to_string()],
            },
        );

        // Input component
        components.insert(
            "input".to_string(),
            ComponentInfo {
                name: "input".to_string(),
                description: "Text input with validation support".to_string(),
                version: "0.1.0".to_string(),
                files: vec!["input.rs".to_string()],
                dependencies: vec!["traits".to_string()],
            },
        );

        // Card component
        components.insert(
            "card".to_string(),
            ComponentInfo {
                name: "card".to_string(),
                description: "Card container with header, content, and footer".to_string(),
                version: "0.1.0".to_string(),
                files: vec!["card.rs".to_string()],
                dependencies: vec![],
            },
        );

        // Dialog component
        components.insert(
            "dialog".to_string(),
            ComponentInfo {
                name: "dialog".to_string(),
                description: "Modal dialog with overlay".to_string(),
                version: "0.1.0".to_string(),
                files: vec!["dialog.rs".to_string()],
                dependencies: vec![],
            },
        );

        // Checkbox component
        components.insert(
            "checkbox".to_string(),
            ComponentInfo {
                name: "checkbox".to_string(),
                description: "Checkbox input component".to_string(),
                version: "0.1.0".to_string(),
                files: vec!["checkbox.rs".to_string()],
                dependencies: vec!["traits".to_string()],
            },
        );

        // Badge component
        components.insert(
            "badge".to_string(),
            ComponentInfo {
                name: "badge".to_string(),
                description: "Badge component for labels and tags".to_string(),
                version: "0.1.0".to_string(),
                files: vec!["badge.rs".to_string()],
                dependencies: vec![],
            },
        );

        // Traits (utility)
        components.insert(
            "traits".to_string(),
            ComponentInfo {
                name: "traits".to_string(),
                description: "Common traits used by components".to_string(),
                version: "0.1.0".to_string(),
                files: vec!["traits.rs".to_string()],
                dependencies: vec![],
            },
        );

        // Prelude (utility)
        components.insert(
            "prelude".to_string(),
            ComponentInfo {
                name: "prelude".to_string(),
                description: "Common imports and utilities".to_string(),
                version: "0.1.0".to_string(),
                files: vec!["prelude.rs".to_string()],
                dependencies: vec![],
            },
        );

        Self { components }
    }

    pub fn get_component(&self, name: &str) -> Result<&ComponentInfo> {
        self.components
            .get(name)
            .context(format!("Component '{}' not found", name))
    }

    pub fn list_components(&self) -> Vec<&ComponentInfo> {
        let mut components: Vec<&ComponentInfo> = self
            .components
            .values()
            .filter(|c| c.name != "traits" && c.name != "prelude")
            .collect();
        components.sort_by(|a, b| a.name.cmp(&b.name));
        components
    }

    pub fn resolve_dependencies(&self, component_name: &str) -> Result<Vec<String>> {
        let mut resolved = Vec::new();
        let mut to_process = vec![component_name.to_string()];
        let mut seen = std::collections::HashSet::new();

        while let Some(name) = to_process.pop() {
            if seen.contains(&name) {
                continue;
            }
            seen.insert(name.clone());

            let component = self.get_component(&name)?;

            // Add dependencies to process list (in reverse order so they're processed first)
            for dep in component.dependencies.iter().rev() {
                if !seen.contains(dep) {
                    to_process.push(dep.clone());
                }
            }

            resolved.push(name);
        }

        // Reverse so dependencies come first
        resolved.reverse();

        Ok(resolved)
    }

}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_dependencies() {
        let registry = Registry::new();
        let deps = registry.resolve_dependencies("button").unwrap();
        assert!(deps.contains(&"traits".to_string()));
        assert!(deps.contains(&"button".to_string()));
        // traits should come before button
        let traits_idx = deps.iter().position(|d| d == "traits").unwrap();
        let button_idx = deps.iter().position(|d| d == "button").unwrap();
        assert!(traits_idx < button_idx);
    }

    #[test]
    fn test_list_components() {
        let registry = Registry::new();
        let components = registry.list_components();
        // Should not include traits and prelude
        assert!(!components.iter().any(|c| c.name == "traits"));
        assert!(!components.iter().any(|c| c.name == "prelude"));
    }
}

