# GPUI-UI: A shadcn-inspired Component Library for GPUI

> Copy-paste component library for GPUI applications. Build your UI with composable, customizable components that you own.

## 🎯 Project Vision

Create a shadcn-style CLI tool that provides high-quality, reusable UI components for GPUI applications. Instead of installing components as dependencies, developers copy the source code directly into their projects, giving them full ownership and customization freedom.

## 🔍 Background

### What is GPUI?

GPUI is a fast, GPU-accelerated UI framework for Rust created by the Zed team. It features:
- Declarative API with builder pattern (`.flex()`, `.bg()`, `.border_1()`)
- Metal rendering on macOS for high performance
- State management through context system
- Reactive UI updates

### The Problem

- GPUI has minimal built-in components
- Zed's `ui` crate is GPL-licensed and Zed-specific
- No general-purpose component library exists
- High barrier to entry for new GPUI developers

### The Solution

A CLI tool that copies battle-tested component source code directly into your project, similar to how shadcn/ui works for React.

## 🏗️ Architecture

### Project Structure

```
gpui-ui/
├── cli/                    # CLI tool (Rust)
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/      # init, add, list, update
│   │   ├── registry.rs    # Component registry logic
│   │   └── config.rs      # Config file management
│   └── Cargo.toml
├── registry/              # Component source files
│   ├── ui/
│   │   ├── button.rs
│   │   ├── input.rs
│   │   ├── card.rs
│   │   ├── dialog.rs
│   │   ├── select.rs
│   │   ├── checkbox.rs
│   │   ├── badge.rs
│   │   └── toast.rs
│   └── metadata.json      # Component metadata
├── examples/              # Live examples
├── docs/                  # Documentation site
└── README.md
```

### Configuration File (`gpui-ui.json`)

```json
{
  "component_path": "src/components/ui",
  "utils_path": "src/lib",
  "gpui_version": "0.1.0",
  "style": {
    "colors": {
      "primary": "rgb(0x3b82f6)",
      "secondary": "rgb(0x64748b)",
      "destructive": "rgb(0xef4444)",
      "muted": "rgb(0xf1f5f9)",
      "accent": "rgb(0xf0f9ff)"
    },
    "radius": "px(4.0)"
  }
}
```

## 🚀 CLI Commands

### Initialize Project
```bash
# Initialize gpui-ui in your project
gpui-ui init

# Options:
#   --path <path>      Specify component path
#   --force            Overwrite existing config
```

### Add Components
```bash
# Add single component
gpui-ui add button

# Add multiple components
gpui-ui add button input card

# Add all components
gpui-ui add --all
```

### List Components
```bash
# List all available components
gpui-ui list

# Show component details
gpui-ui info button
```

### Update Components
```bash
# Update specific component
gpui-ui update button

# Update all components
gpui-ui update --all
```

## 📦 Core Components (Phase 1)

### 1. Button
**Variants:** Default, Destructive, Outline, Ghost, Link
**Props:** label, variant, size, disabled, on_click

```rust
Button::new("Click me")
    .variant(ButtonVariant::Default)
    .size(ButtonSize::Medium)
    .on_click(|_, cx| {
        // Handle click
    })
```

### 2. Input
**Types:** Text, Password, Email, Number
**Features:** Validation states, icons, placeholder

```rust
Input::new()
    .placeholder("Enter text...")
    .value(&state.value)
    .on_change(|value, cx| {
        // Handle change
    })
```

### 3. Card
**Parts:** Header, Content, Footer
**Features:** Elevation, borders, padding

```rust
Card::new()
    .header(CardHeader::new("Title"))
    .content(/* content */)
    .footer(/* actions */)
```

### 4. Dialog/Modal
**Features:** Overlay, focus trap, esc to close
**Parts:** Trigger, Content, Header, Footer

### 5. Select
**Features:** Single/multi select, search, keyboard navigation

### 6. Checkbox & Radio
**Features:** Labels, disabled state, indeterminate

### 7. Badge
**Variants:** Default, Secondary, Destructive, Outline

### 8. Toast
**Features:** Auto-dismiss, positions, variants

## 🎨 Design System

### Color Scheme
- **Primary:** Main brand color
- **Secondary:** Complementary actions
- **Destructive:** Dangerous actions
- **Muted:** Background elements
- **Accent:** Highlights and emphasis

### Spacing Scale
```rust
// GPUI pixel values
px(4.0)   // xs
px(8.0)   // sm
px(12.0)  // md
px(16.0)  // lg
px(24.0)  // xl
px(32.0)  // 2xl
```

### Typography
- **Display:** Large headings
- **Heading:** Section titles
- **Body:** Regular text
- **Small:** Captions and labels
- **Mono:** Code and technical content

## 🛠️ Technical Implementation

### Component Template Structure

```rust
use gpui::*;

#[derive(Clone)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Ghost,
    Link,
}

#[derive(Clone)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

pub struct Button {
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&mut Window, &mut Context<Self>)>>,
}

impl Button {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Medium,
            disabled: false,
            on_click: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&mut Window, &mut Context<Self>) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn get_styles(&self) -> ButtonStyles {
        match self.variant {
            ButtonVariant::Default => ButtonStyles {
                bg: rgb(0x3b82f6),
                text: rgb(0xffffff),
                hover_bg: rgb(0x2563eb),
                border: None,
            },
            ButtonVariant::Destructive => ButtonStyles {
                bg: rgb(0xef4444),
                text: rgb(0xffffff),
                hover_bg: rgb(0xdc2626),
                border: None,
            },
            // ... other variants
        }
    }
}

impl Render for Button {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let styles = self.get_styles();
        let (height, px_size, text_size) = match self.size {
            ButtonSize::Small => (px(32.0), px(12.0), TextSize::Small),
            ButtonSize::Medium => (px(40.0), px(16.0), TextSize::Default),
            ButtonSize::Large => (px(48.0), px(24.0), TextSize::Large),
        };

        div()
            .flex()
            .items_center()
            .justify_center()
            .h(height)
            .px(px_size)
            .bg(styles.bg)
            .text_color(styles.text)
            .rounded(px(4.0))
            .cursor_pointer()
            .when(self.disabled, |div| {
                div.opacity(0.5).cursor_not_allowed()
            })
            .hover(|style| style.bg(styles.hover_bg))
            .child(self.label.clone())
    }
}

struct ButtonStyles {
    bg: Rgb,
    text: Rgb,
    hover_bg: Rgb,
    border: Option<Rgb>,
}
```

### CLI Implementation (Rust)

**Dependencies:**
- `clap` - Command-line parsing
- `serde` / `serde_json` - Config serialization
- `tokio` - Async runtime
- `reqwest` - HTTP client for fetching components
- `colored` - Terminal colors
- `indicatif` - Progress bars

**Key Features:**
- Component registry with versioning
- Dependency resolution
- Conflict detection
- Update checking
- Interactive prompts

## 📋 Development Roadmap

### Phase 1: Foundation (Weeks 1-2)
- [x] Project research and planning
- [ ] CLI architecture design
- [ ] Set up project structure
- [ ] Implement `init` command
- [ ] Create configuration system
- [ ] Design component metadata format

### Phase 2: Core Components (Weeks 3-4)
- [ ] Implement Button component
- [ ] Implement Input component
- [ ] Implement Card component
- [ ] Create component examples
- [ ] Write component tests

### Phase 3: CLI Features (Weeks 5-6)
- [ ] Implement `add` command
- [ ] Implement `list` command
- [ ] Implement `update` command
- [ ] Add dependency resolution
- [ ] Create progress indicators

### Phase 4: Extended Components (Weeks 7-8)
- [ ] Implement Dialog/Modal
- [ ] Implement Select
- [ ] Implement Checkbox/Radio
- [ ] Implement Badge
- [ ] Implement Toast

### Phase 5: Polish & Documentation (Weeks 9-10)
- [ ] Create documentation site
- [ ] Write comprehensive guides
- [ ] Add more examples
- [ ] Set up CI/CD
- [ ] Create demo applications

### Phase 6: Community & Growth (Weeks 11-12)
- [ ] Release v0.1.0
- [ ] Share in Zed Discord
- [ ] Create video tutorials
- [ ] Gather community feedback
- [ ] Plan v0.2.0 features

## 🎯 Success Metrics

- **Component Coverage:** 15+ components by v1.0
- **Adoption:** 50+ projects using gpui-ui
- **Documentation:** 100% component coverage
- **Performance:** All components render at 60fps
- **Community:** Active Discord/GitHub discussions

## 🤝 Contributing

This project aims to become a community-driven effort. Contributions welcome for:
- New components
- Bug fixes
- Documentation improvements
- Example applications
- Testing and feedback

## 📚 Resources

- **GPUI Documentation:** https://www.gpui.rs/
- **Zed Source Code:** https://github.com/zed-industries/zed
- **GPUI Discord:** Join Zed Discord for GPUI discussions
- **shadcn/ui (inspiration):** https://ui.shadcn.com/

## ⚖️ License

Components are provided as source code under Apache 2.0 license, giving you full ownership and freedom to modify.

## 🙏 Acknowledgments

- Zed team for creating GPUI
- shadcn for the CLI inspiration
- Rust community for excellent tooling

---

**Status:** 🚧 Planning Phase
**Version:** 0.0.1-alpha
**Last Updated:** October 2025
