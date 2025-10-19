# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

GPUI-UI is a shadcn-inspired component library for GPUI applications. It's a CLI tool that copies component source code directly into user projects (not a dependency), giving developers full ownership and customization freedom.

### Key Concepts

- **GPUI**: A fast, GPU-accelerated UI framework for Rust by the Zed team, featuring declarative API with builder pattern (`.flex()`, `.bg()`, `.border_1()`), Metal rendering on macOS, and reactive UI updates
- **Copy-paste philosophy**: Like shadcn/ui, components are copied into projects, not installed as dependencies
- **Component ownership**: Developers get full source code and customization control

## Repository Structure

This is a Cargo workspace with three main parts:

```
gpui-ui/
├── Cargo.toml           # Workspace root
├── cli/                 # CLI tool for managing components
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── components/          # Reusable GPUI component library
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── src/                 # Workspace-level utilities
    └── lib.rs
```

**Workspace members:**
- `cli` - The `gpui-ui` CLI tool (commands: init, add, list, update)
- `components` - Component source files that get copied into user projects

## Development Commands

### Building

```bash
# Build entire workspace
cargo build

# Build specific workspace member
cargo build -p cli
cargo build -p components

# Build with release optimizations
cargo build --release
```

### Testing

```bash
# Run all tests in workspace
cargo test

# Run tests for specific package
cargo test -p cli
cargo test -p components

# Run specific test
cargo test --test test_name

# Run tests with output
cargo test -- --nocapture
```

### Running the CLI

```bash
# Run CLI during development
cargo run -p cli

# Run with arguments
cargo run -p cli -- init
cargo run -p cli -- add button
cargo run -p cli -- list
```

### Linting and Formatting

```bash
# Format code
cargo fmt

# Check formatting without making changes
cargo fmt -- --check

# Run clippy linter
cargo clippy

# Clippy with strict warnings
cargo clippy -- -D warnings
```

## Accessing GPUI Documentation (For AI Agents)

When working with GPUI, here are the best ways to access documentation and examples:

### Local GPUI Examples

The GPUI crate includes extensive examples that are invaluable for understanding the API:

```bash
# Find the GPUI source location
find ~/.cargo/registry/src -name "gpui-0.2.1" -type d

# Location (typically):
# ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gpui-0.2.1/

# Examples are in:
# ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gpui-0.2.1/examples/
```

**Key examples to reference:**
- `hello_world.rs` - Basic window and rendering setup
- `window.rs` - WindowOptions and window management
- `input.rs` - Input handling and interactive components
- `animation.rs` - Animations and transitions
- `scrollable.rs` - Scrollable content

**To read an example:**
```bash
# Use the read_file tool with absolute path:
read_file("/Users/USER/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gpui-0.2.1/examples/hello_world.rs")
```

### GPUI API Patterns (v0.2.1)

Based on examples, the correct patterns are:

**Application initialization:**
```rust
use gpui::{App, Application, Bounds, Context, Window, WindowOptions, div, prelude::*, px, size};

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| MyComponent::new()),
        ).unwrap();
        cx.activate(true);
    });
}
```

**Component rendering:**
- Components implement `Render` trait with `fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement`
- To make components usable as children, implement `IntoElement` instead with `type Element = Div` and `fn into_element(self) -> Self::Element`

**Required imports:**
```rust
use gpui::*;
use gpui::prelude::*;  // IMPORTANT: Provides FluentBuilder trait for .when() and other methods
```

### Online Resources

1. **Official Documentation**: https://docs.rs/gpui/0.2.1/gpui/
2. **GPUI Book**: https://matinaniss.github.io/gpui-book/
3. **GPUI Tutorial**: https://github.com/hedge-ops/gpui-tutorial
4. **Official Website**: https://www.gpui.rs/

### Common Issues

1. **"method not found" for `.when()`**: Make sure to import `use gpui::prelude::*;`
2. **Components not working as children**: Implement `IntoElement` instead of just `Render`
3. **Wrong App initialization**: Use `Application::new().run()` not `App::new().run()`
4. **WindowOptions fields**: Use `..Default::default()` to avoid specifying all fields

### Development Workflow

1. **Always use `cargo check`** instead of `cargo build` for faster iteration
2. **Reference GPUI examples** when implementing new patterns
3. **Read actual source code** from the cargo registry when documentation is unclear
4. **Test with minimal examples** before building complex components

## Architecture Details

### CLI Implementation (cli/)

The CLI is implemented using:
- `clap` for command-line parsing with derive macros
- `serde`/`serde_json` for config serialization
- `tokio` for async runtime
- `reqwest` for fetching components from registry
- `colored` for terminal output
- `indicatif` for progress bars

**Planned CLI commands:**
- `init` - Initialize gpui-ui in a project, creates `gpui-ui.json` config
- `add` - Copy components from registry to user's project
- `list` - Display available components
- `update` - Update components to latest versions
- `info` - Show component details

**Key responsibilities:**
- Component registry management and versioning
- Dependency resolution between components
- Conflict detection (avoiding overwrites)
- Configuration file management (`gpui-ui.json`)

### Component Library (components/)

Contains battle-tested GPUI components that users copy into their projects.

**Planned Phase 1 components:**
- Button (variants: Default, Destructive, Outline, Ghost, Link)
- Input (types: Text, Password, Email, Number with validation)
- Card (parts: Header, Content, Footer)
- Dialog/Modal (with overlay, focus trap)
- Select (single/multi-select, keyboard navigation)
- Checkbox & Radio
- Badge
- Toast (auto-dismiss, positioning)

**Component structure pattern:**
```rust
use gpui::*;

// Enums for variants/sizes
#[derive(Clone)]
pub enum ComponentVariant { /* ... */ }

// Main component struct
pub struct Component {
    // Props
}

impl Component {
    pub fn new() -> Self { /* ... */ }

    // Builder methods
    pub fn variant(mut self, variant: ComponentVariant) -> Self { /* ... */ }

    // Style computation
    fn get_styles(&self) -> ComponentStyles { /* ... */ }
}

// GPUI rendering
impl Render for Component {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            // ... GPUI styling chain
    }
}
```

### Configuration System

User projects have a `gpui-ui.json` file:
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

### Design System

**Color scheme:**
- Primary: Main brand color (default: blue)
- Secondary: Complementary actions (default: slate)
- Destructive: Dangerous actions (default: red)
- Muted: Background elements (default: light gray)
- Accent: Highlights and emphasis (default: light blue)

**Spacing scale (GPUI pixel values):**
- `px(4.0)` - xs
- `px(8.0)` - sm
- `px(12.0)` - md
- `px(16.0)` - lg
- `px(24.0)` - xl
- `px(32.0)` - 2xl

**GPUI styling patterns:**
- Use builder pattern: `.flex().items_center().justify_center()`
- Colors via `rgb(0xHEXCODE)`
- Conditional styling via `.when(condition, |style| style.foo())`
- Hover states via `.hover(|style| style.bar())`

## Important Notes

### Workspace Configuration

- **Rust version:** 1.75+ (as specified in workspace Cargo.toml)
- **Edition:** 2021 for workspace, but individual packages show "2024" (likely placeholder, should be 2021)
- **GPUI version:** 0.2.1 (workspace dependency)
- **Resolver:** Version 2 for Cargo workspace

### GPUI-Specific Patterns

When working with GPUI components:
1. State management uses GPUI's context system
2. Rendering is declarative via `Render` trait
3. All visual elements built with `div()` and styling chains
4. Event handlers receive `&mut Window` and `&mut Context<Self>`
5. Use `SharedString` for text content
6. Use `IntoElement` trait for composability

### Development Workflow

1. Components are developed in `components/src/`
2. CLI tool in `cli/src/` manages copying components to user projects
3. When adding new components, update component metadata for registry
4. Follow the component template structure for consistency
5. Test components with example applications

## Project Status

**Current phase:** Planning/Foundation (v0.0.1-alpha)

The project is in early development. The basic workspace structure exists, but most functionality is still to be implemented according to the roadmap in README.md.
