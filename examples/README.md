# GPUI-UI Examples

This directory contains example applications demonstrating how to use the GPUI-UI component library.

## Running Examples

Each example is a separate Rust binary in the workspace. Run them with:

```bash
# From the workspace root
cargo run -p <example-name>

# For example:
cargo run -p todo-app
```

## Available Examples

### Todo App

**Path:** `examples/todo-app/`  
**Description:** A simple todo list application showcasing:
- Button component integration from `gpui-ui-components`
- Different button variants (Default, Destructive, Ghost, Outline)
- Button sizes (Small, Medium, Large)
- Basic GPUI layout patterns (flex, grid, colors)
- Static UI rendering (interactive features to be added)

**Run:**
```bash
cargo run -p todo-app
```

**Key learnings:**
- How to import and use components from the `gpui-ui-components` crate
- Proper GPUI application initialization with `Application::new().run()`
- Using `IntoElement` trait to make custom components work as children
- GPUI styling patterns with the builder API

## Component Integration

The examples demonstrate the "import and use" philosophy of gpui-ui components:

```rust
use gpui_ui_components::{Button, button::{ButtonVariant, ButtonSize}};

// Use in your render function
Button::new("Click me")
    .variant(ButtonVariant::Default)
    .size(ButtonSize::Medium)
```

Since components are meant to be copied into user projects eventually (via the CLI), these examples show both:
1. **Direct import** - Using components as a dependency (current examples)
2. **Copy pattern** - What it looks like when components are copied into a project (future examples)

## Adding New Examples

To add a new example:

1. Create a new directory: `examples/your-example/`
2. Add `Cargo.toml`:
   ```toml
   [package]
   name = "your-example"
   version.workspace = true
   edition.workspace = true
   publish = false

   [[bin]]
   name = "your-example"
   path = "src/main.rs"

   [dependencies]
   gpui.workspace = true
   gpui-ui-components = { path = "../../components" }
   ```
3. Add to workspace members in root `Cargo.toml`:
   ```toml
   members = ["cli", "components", "examples/your-example"]
   ```
4. Create `src/main.rs` with your example
5. Document it in this README

## Best Practices

When creating examples:

- **Keep them focused** - Each example should demonstrate specific components or patterns
- **Add comments** - Explain what each part does, especially GPUI-specific patterns
- **Show variants** - Demonstrate different component variants and configurations
- **Real-world scenarios** - Create examples that resemble actual application use cases
- **Progressive complexity** - Start simple, build up to more complex patterns

## Testing Component Integration

These examples serve as integration tests for the component library. They verify:

✅ Components compile correctly  
✅ Components render in GPUI applications  
✅ Builder patterns work as expected  
✅ Styling is applied correctly  
✅ Components can be composed together  

Run `cargo check` on examples to quickly verify component changes don't break integration.

