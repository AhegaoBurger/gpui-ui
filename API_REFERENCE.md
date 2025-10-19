# GPUI-UI Component API Reference

Quick reference for using the refactored Phase 1 components.

## Button

### Basic Usage
```rust
Button::new("my-button", "Click me")
    .variant(ButtonVariant::Default)
    .size(ButtonSize::Medium)
    .on_click(|event, window, cx| {
        println!("Clicked!");
        cx.notify();
    })
```

### Variants
- `ButtonVariant::Default` - Primary blue button
- `ButtonVariant::Destructive` - Red danger button
- `ButtonVariant::Outline` - Outlined button
- `ButtonVariant::Ghost` - Transparent button
- `ButtonVariant::Link` - Text link style

### Sizes
- `ButtonSize::Small`
- `ButtonSize::Medium`
- `ButtonSize::Large`

### Methods
- `.variant(ButtonVariant)` - Set visual style
- `.size(ButtonSize)` - Set button size
- `.disabled(bool)` - Enable/disable button
- `.on_click(handler)` - Set click handler

---

## Checkbox

### Basic Usage
```rust
Checkbox::new("my-checkbox", ToggleState::Selected)
    .size(CheckboxSize::Medium)
    .label("Enable feature")
    .on_click(|new_state, window, cx| {
        println!("New state: {:?}", new_state);
        cx.notify();
    })
```

### Toggle State
- `ToggleState::Unselected` - Not checked
- `ToggleState::Selected` - Checked
- `ToggleState::Indeterminate` - Partial selection (e.g., "select all")

Convert from bool: `ToggleState::from(true)`

### Sizes
- `CheckboxSize::Small`
- `CheckboxSize::Medium`
- `CheckboxSize::Large`

### Methods
- `.size(CheckboxSize)` - Set checkbox size
- `.label(text)` - Add label next to checkbox
- `.disabled(bool)` - Enable/disable checkbox
- `.on_click(handler)` - Set click handler (receives new state)
- `.toggle_state(ToggleState)` - Set initial state

### Helper Constructor
```rust
Checkbox::checked("my-checkbox", true)  // Convenience for bool -> ToggleState
```

---

## Input

### Basic Usage
```rust
Input::new()
    .id("my-input")
    .placeholder("Enter text...")
    .size(InputSize::Medium)
    .variant(InputVariant::Default)
    .value("Initial value")
```

### Types
- `InputType::Text`
- `InputType::Password`
- `InputType::Email`
- `InputType::Number`
- `InputType::Search`

### Variants
- `InputVariant::Default` - Standard input
- `InputVariant::Error` - Error state (red border)
- `InputVariant::Success` - Success state (green border)

### Sizes
- `InputSize::Small`
- `InputSize::Medium`
- `InputSize::Large`

### Methods
- `.id(ElementId)` - Set element ID (optional)
- `.input_type(InputType)` - Set input type
- `.size(InputSize)` - Set input size
- `.variant(InputVariant)` - Set visual state
- `.placeholder(text)` - Set placeholder text
- `.value(text)` - Set current value
- `.label(text)` - Add label above input
- `.error(text)` - Set error message (also sets variant to Error)
- `.disabled(bool)` - Enable/disable input
- `.required(bool)` - Mark as required (shows *)

**Note:** Currently visual representation only. Full text editing requires focus management.

---

## Badge

### Basic Usage
```rust
Badge::new("5 items")
    .variant(BadgeVariant::Primary)
    .size(BadgeSize::Medium)
    .dot(true)
```

### Variants
- `BadgeVariant::Default` - Gray badge
- `BadgeVariant::Primary` - Blue badge
- `BadgeVariant::Secondary` - Slate badge
- `BadgeVariant::Success` - Green badge
- `BadgeVariant::Warning` - Amber badge
- `BadgeVariant::Error` - Red badge
- `BadgeVariant::Outline` - Outlined badge

### Sizes
- `BadgeSize::Small`
- `BadgeSize::Medium`
- `BadgeSize::Large`

### Methods
- `.variant(BadgeVariant)` - Set visual style
- `.size(BadgeSize)` - Set badge size
- `.dot(bool)` - Show/hide colored dot indicator

---

## Card

### Basic Usage
```rust
Card::new()
    .variant(CardVariant::Elevated)
    .child(
        CardHeader::new()
            .title("Card Title")
            .description("Card description")
    )
    .child(
        CardContent::new()
            .child(div().child("Card body content"))
    )
    .child(
        CardFooter::new()
            .child(Button::new("action", "Action"))
    )
```

### Variants
- `CardVariant::Elevated` - Shadow effect (simulated with border)
- `CardVariant::Outlined` - Border outline
- `CardVariant::Filled` - Filled background

### Subcomponents

#### CardHeader
```rust
CardHeader::new()
    .title("Title")
    .description("Description")
```

#### CardContent
```rust
CardContent::new()
    .child(element1)
    .children(vec![element2, element3])
```

#### CardFooter
```rust
CardFooter::new()
    .child(button)
    .children(vec![button1, button2])
```

### Convenience Methods
```rust
Card::new()
    .header(CardHeader::new().title("Title"))  // Shorthand
    .content(CardContent::new().child(content))
    .footer(CardFooter::new().child(button))
```

---

## Dialog

### Basic Usage
```rust
Dialog::new()
    .size(DialogSize::Medium)
    .open(true)
    .child(
        DialogHeader::new()
            .title("Dialog Title")
            .description("Dialog description")
    )
    .child(
        DialogContent::new()
            .child(div().child("Dialog body"))
    )
    .child(
        DialogFooter::new()
            .child(Button::new("cancel", "Cancel"))
            .child(Button::new("confirm", "Confirm"))
    )
```

### Sizes
- `DialogSize::Small` - 400px wide
- `DialogSize::Medium` - 500px wide
- `DialogSize::Large` - 700px wide
- `DialogSize::Full` - 900px wide

### Methods
- `.size(DialogSize)` - Set dialog width
- `.open(bool)` - Show/hide dialog (renders empty div when closed)
- `.child(element)` - Add content

### Subcomponents

#### DialogHeader
```rust
DialogHeader::new()
    .title("Title")
    .description("Description")
```

#### DialogContent
```rust
DialogContent::new()
    .child(element)
    .children(vec![element1, element2])
```

#### DialogFooter
```rust
DialogFooter::new()
    .child(button)
    .children(vec![cancel_button, confirm_button])
```

### Convenience Methods
```rust
Dialog::new()
    .header(DialogHeader::new().title("Title"))
    .content(DialogContent::new().child(content))
    .footer(DialogFooter::new().child(button))
```

---

## Common Patterns

### Event Handlers with State Updates

```rust
Button::new("my-btn", "Click")
    .on_click(cx.listener(|this, _event, _window, cx| {
        // Mutate component state
        this.some_field = new_value;
        
        // Trigger re-render
        cx.notify();
    }))
```

### Conditional Rendering

```rust
div()
    .when(show_button, |d| {
        d.child(Button::new("btn", "Click me"))
    })
    .when_some(optional_value, |d, value| {
        d.child(Badge::new(value))
    })
```

### Element IDs

Element IDs can be created several ways:

```rust
// String literal (most common)
Button::new("my-button", "Click")

// From string
let id = "my-button".to_string();
Button::new(id, "Click")

// ElementId enum
Button::new(ElementId::Name("my-button".into()), "Click")

// Dynamic IDs for lists
let id = format!("button-{}", index);
Button::new(id, "Click")
```

### Styling Components

Components return `Div` elements that can be further styled:

```rust
// Wrap in div for additional styling
div()
    .w_full()
    .p_4()
    .child(
        Button::new("btn", "Click")
    )
```

---

## Import Guide

### Minimal imports
```rust
use gpui::*;
use gpui::prelude::*;  // Essential for .when() and other methods
use gpui_ui_components::*;
```

### Specific imports
```rust
use gpui::{App, Application, Window, Context, div, px};
use gpui::prelude::*;
use gpui_ui_components::{
    Button, ButtonVariant, ButtonSize,
    Checkbox, ToggleState, CheckboxSize,
    Input, InputSize, InputVariant,
    Badge, BadgeSize, BadgeVariant,
    Card, CardHeader, CardContent, CardFooter, CardVariant,
    Dialog, DialogHeader, DialogContent, DialogFooter, DialogSize,
};
```

---

## Tips & Tricks

### 1. Always provide ElementIds for interactive components
```rust
// ✅ Good
Button::new("save-btn", "Save")

// ❌ Bad (won't compile now)
Button::new("Save")  // Missing ElementId
```

### 2. Use cx.listener for state mutations
```rust
// ✅ Good - captures context
Button::new("btn", "Click")
    .on_click(cx.listener(|this, event, window, cx| {
        this.update_state();
        cx.notify();
    }))

// ❌ Bad - loses access to component state
Button::new("btn", "Click")
    .on_click(|event, window, cx| {
        // Can't access component state here!
    })
```

### 3. Remember to call cx.notify()
```rust
.on_click(cx.listener(|this, _, _, cx| {
    this.value = new_value;
    cx.notify();  // ✅ Triggers re-render
}))
```

### 4. Use ToggleState for checkboxes
```rust
// ✅ Good - proper type
Checkbox::new("cb", ToggleState::from(is_checked))

// ❌ Bad - bool not supported anymore
Checkbox::new("cb", is_checked)  // Won't compile
```

### 5. Wrap Input for flex layouts
```rust
// ✅ Good - wraps Input in flex container
div()
    .flex_1()
    .child(Input::new().id("input"))

// ❌ Bad - Input doesn't have flex_1
Input::new().flex_1()  // Won't work
```

---

## Complete Example

```rust
use gpui::*;
use gpui::prelude::*;
use gpui_ui_components::*;

struct MyApp {
    count: usize,
    enabled: bool,
    input_value: String,
}

impl Render for MyApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Card::new()
            .variant(CardVariant::Elevated)
            .header(
                CardHeader::new()
                    .title("Counter App")
                    .description("A simple counter with state")
            )
            .content(
                CardContent::new()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(
                                Badge::new(format!("Count: {}", self.count))
                                    .variant(BadgeVariant::Primary)
                            )
                            .child(
                                Checkbox::new("enable", ToggleState::from(self.enabled))
                                    .label("Enable counting")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.enabled = !this.enabled;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Input::new()
                                    .id("input")
                                    .label("Name")
                                    .placeholder("Enter your name")
                                    .value(self.input_value.clone())
                            )
                    )
            )
            .footer(
                CardFooter::new()
                    .child(
                        Button::new("increment", "Increment")
                            .variant(ButtonVariant::Default)
                            .disabled(!self.enabled)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.count += 1;
                                cx.notify();
                            }))
                    )
                    .child(
                        Button::new("reset", "Reset")
                            .variant(ButtonVariant::Destructive)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.count = 0;
                                cx.notify();
                            }))
                    )
            )
    }
}
```

---

## Need Help?

- See `examples/todo-app/src/main.rs` for a complete working example
- Check `ZEDCOMPARISON.md` for detailed pattern explanations
- Review `REFACTOR_SUMMARY.md` for architectural overview
- Look at Zed's UI components in `/ui/src/components/` for advanced patterns

