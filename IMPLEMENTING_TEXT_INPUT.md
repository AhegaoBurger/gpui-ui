# Implementing a Proper Text Input Component in GPUI

## Current State

Our current `Input` component is **visual only** - it displays an input field but doesn't actually accept keyboard input. This is why the todo app uses an "Add Random Todo" button instead of letting you type.

## Why Text Input is Complex

A fully functional text input requires several sophisticated features that go beyond simple button clicks:

### 1. Focus Management
- Tracking when the input is "active" (focused)
- Handling click events to gain focus
- Handling blur events to lose focus
- Visual indication of focus state
- Tab navigation between inputs

### 2. Keyboard Event Handling
- Capturing key press events
- Converting key codes to characters
- Handling special keys (Enter, Backspace, Delete, Arrow keys)
- Modifier key handling (Cmd+A, Cmd+C, Cmd+V)

### 3. Text Editing State
- Current text content
- Cursor position (insertion point)
- Text selection (start and end positions)
- Undo/redo history
- Clipboard integration

### 4. Visual Rendering
- Rendering text content
- Rendering cursor (blinking line)
- Rendering text selection (highlighted region)
- Scrolling for overflow text
- Placeholder text when empty

### 5. Text Operations
- Insert character at cursor
- Delete character (backspace, delete)
- Move cursor (arrow keys, home, end)
- Select text (shift + arrows)
- Copy/paste/cut operations

## GPUI's Text Editing Primitives

GPUI provides low-level text editing support, but it's not as high-level as HTML `<input>`. Here's what GPUI offers:

### Key GPUI Types

```rust
// From GPUI documentation/source
use gpui::{
    FocusHandle,        // Tracks focus state
    KeyDownEvent,       // Keyboard input events
    TextRun,            // Styled text segments
    ShapedLine,         // Rendered text layout
    Pixels,             // For positioning
};
```

### Focus System

```rust
pub struct TextInput {
    focus_handle: FocusHandle,
    // ...
}

impl TextInput {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

// In render:
div()
    .id(self.id)
    .track_focus(&self.focus_handle)
    .on_mouse_down(cx.listener(|this, event, cx| {
        this.focus_handle.focus(cx);
    }))
    .on_blur(cx.listener(|this, event, cx| {
        // Lost focus
    }))
```

### Keyboard Event Handling

```rust
// In render:
div()
    .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
        if !this.focus_handle.is_focused(cx) {
            return;
        }

        match event.keystroke.key.as_ref() {
            "backspace" => {
                this.delete_backward(cx);
            }
            "delete" => {
                this.delete_forward(cx);
            }
            "left" => {
                this.move_cursor_left(cx);
            }
            "right" => {
                this.move_cursor_right(cx);
            }
            "enter" => {
                if let Some(on_submit) = &this.on_submit {
                    on_submit(&this.text, window, cx);
                }
            }
            _ => {
                // Regular character input
                if let Some(text) = &event.keystroke.ime_key {
                    this.insert_text(text, cx);
                }
            }
        }
    }))
```

## Implementation Approaches

### Approach 1: Basic Single-Line Input (Recommended First Step)

Implement a simple single-line text input with:
- Focus state
- Basic keyboard input (typing, backspace)
- Cursor rendering (no selection yet)
- Submit on Enter key

**Estimated Complexity:** Medium (3-5 hours)
**Functionality:** ~60% of a full input

```rust
#[derive(IntoElement)]
pub struct WorkingInput {
    id: ElementId,
    focus_handle: FocusHandle,
    text: SharedString,
    cursor_position: usize,
    placeholder: Option<SharedString>,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_submit: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl WorkingInput {
    pub fn new(id: impl Into<ElementId>, cx: &mut App) -> Self {
        Self {
            id: id.into(),
            focus_handle: cx.focus_handle(),
            text: "".into(),
            cursor_position: 0,
            placeholder: None,
            on_change: None,
            on_submit: None,
        }
    }

    fn insert_char(&mut self, c: char, cx: &mut App) {
        let mut text = self.text.to_string();
        text.insert(self.cursor_position, c);
        self.text = text.into();
        self.cursor_position += 1;
        
        if let Some(on_change) = &self.on_change {
            on_change(&self.text, cx);
        }
        cx.notify();
    }

    fn delete_backward(&mut self, cx: &mut App) {
        if self.cursor_position > 0 {
            let mut text = self.text.to_string();
            text.remove(self.cursor_position - 1);
            self.text = text.into();
            self.cursor_position -= 1;
            
            if let Some(on_change) = &self.on_change {
                on_change(&self.text, cx);
            }
            cx.notify();
        }
    }
}

impl RenderOnce for WorkingInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_focused = self.focus_handle.is_focused(cx);
        
        div()
            .id(self.id)
            .track_focus(&self.focus_handle)
            .flex()
            .items_center()
            .px(px(12.0))
            .py(px(8.0))
            .border_1()
            .border_color(if is_focused {
                rgb(0x3b82f6)  // Blue when focused
            } else {
                rgb(0xd1d5db)  // Gray when not focused
            })
            .rounded(px(6.0))
            .bg(rgb(0xffffff))
            .on_mouse_down(cx.listener(|this, _event, window, cx| {
                this.focus_handle.focus(window);
                cx.notify();
            }))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                if !this.focus_handle.is_focused(cx) {
                    return;
                }

                match event.keystroke.key.as_ref() {
                    "backspace" => {
                        this.delete_backward(cx);
                    }
                    "enter" => {
                        if let Some(on_submit) = &this.on_submit {
                            on_submit(&this.text, window, cx);
                        }
                    }
                    _ => {
                        // Handle regular text input
                        if let Some(text) = &event.keystroke.ime_key {
                            for c in text.chars() {
                                this.insert_char(c, cx);
                            }
                        }
                    }
                }
            }))
            .child(
                div()
                    .flex()
                    .items_center()
                    .when(self.text.is_empty() && !is_focused, |d| {
                        d.text_color(rgb(0x9ca3af))
                            .child(self.placeholder.unwrap_or("Type here...".into()))
                    })
                    .when(!self.text.is_empty() || is_focused, |d| {
                        d.child(self.text.clone())
                            // TODO: Add cursor rendering here
                    })
            )
    }
}
```

### Approach 2: Use GPUI's Editor Component

GPUI has an `Editor` component used throughout Zed for text editing. This is a full-featured text editor that supports:
- Multi-line editing
- Syntax highlighting
- Multiple cursors
- Code completion
- And much more

**Pros:** Full-featured, battle-tested
**Cons:** Heavy-weight, complex API, might be overkill for simple inputs

**Location:** Check GPUI source or Zed's codebase for `Editor` component usage.

### Approach 3: Wrap a Native Text Input

Some UI frameworks allow wrapping native platform text inputs. This provides:
- Native OS text input behavior
- IME support for international keyboards
- Native text selection and cursor
- Platform-specific keyboard shortcuts

**Pros:** Native feel, handles complex input scenarios
**Cons:** Platform-specific code, may not match your design perfectly

## Step-by-Step Implementation Guide

### Phase 1: Basic Text Display and Focus

1. Add `FocusHandle` to your Input struct
2. Implement focus on click
3. Show focus indicator (border color change)
4. Test focus/blur behavior

### Phase 2: Keyboard Input

1. Add `on_key_down` handler
2. Handle basic character insertion
3. Handle backspace
4. Update text state and trigger re-render

### Phase 3: Cursor Rendering

1. Track cursor position
2. Render cursor as a vertical line at position
3. Make cursor blink (animation)
4. Update cursor position on arrow keys

### Phase 4: Text Selection

1. Add selection state (start/end positions)
2. Handle shift+arrow for selection
3. Render selection background
4. Implement copy/paste

### Phase 5: Polish

1. Add validation
2. Add error states
3. Add disabled state
4. Add keyboard shortcuts (Cmd+A, etc.)
5. Add placeholder animation
6. Add input masking (for passwords)

## Example Usage (After Implementation)

```rust
// In your app:
struct MyApp {
    input_text: String,
}

impl Render for MyApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child(
                WorkingInput::new("my-input", cx)
                    .placeholder("Enter todo text...")
                    .value(&self.input_text)
                    .on_change(cx.listener(|this, text, _window, cx| {
                        this.input_text = text.to_string();
                        cx.notify();
                    }))
                    .on_submit(cx.listener(|this, text, _window, cx| {
                        this.add_todo(text);
                        this.input_text.clear();
                        cx.notify();
                    }))
            )
    }
}
```

## Resources

### GPUI Documentation
- **Focus System:** https://docs.rs/gpui/latest/gpui/struct.FocusHandle.html
- **Keyboard Events:** https://docs.rs/gpui/latest/gpui/struct.KeyDownEvent.html
- **Text Rendering:** https://docs.rs/gpui/latest/gpui/text/

### Zed Source Code
Look at how Zed implements text editing:
- Search for `Editor` component in Zed's codebase
- Look at `TextField` or similar components
- Check how they handle `KeyDownEvent`

### GPUI Examples
Check GPUI's examples directory:
```bash
~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gpui-0.2.1/examples/
```

Key examples:
- `input.rs` - Keyboard input handling
- `text.rs` - Text rendering
- `focus.rs` - Focus management

## Common Pitfalls

### 1. Forgetting to Call cx.notify()
```rust
// ❌ Bad - UI won't update
fn insert_char(&mut self, c: char) {
    self.text.push(c);
    // Missing cx.notify()!
}

// ✅ Good
fn insert_char(&mut self, c: char, cx: &mut App) {
    self.text.push(c);
    cx.notify();  // Triggers re-render
}
```

### 2. Not Checking Focus State
```rust
// ❌ Bad - handles keys even when not focused
.on_key_down(cx.listener(|this, event, window, cx| {
    this.insert_char(event.key);
}))

// ✅ Good - only handles keys when focused
.on_key_down(cx.listener(|this, event, window, cx| {
    if !this.focus_handle.is_focused(cx) {
        return;  // Ignore if not focused
    }
    this.insert_char(event.key);
}))
```

### 3. Incorrect Cursor Position After Insert
```rust
// ❌ Bad - cursor position gets out of sync
text.insert(cursor_position, c);
// Missing: cursor_position += 1;

// ✅ Good
text.insert(cursor_position, c);
cursor_position += 1;  // Move cursor forward
```

### 4. Not Handling Special Characters
```rust
// ❌ Bad - crashes on emoji or multi-byte chars
text.remove(position);

// ✅ Good - use char boundaries
let mut chars: Vec<char> = text.chars().collect();
chars.remove(position);
text = chars.into_iter().collect();
```

## Testing Strategy

1. **Manual Testing:**
   - Type characters → should appear
   - Press backspace → should delete
   - Press enter → should submit
   - Click outside → should lose focus
   - Tab between inputs → should work

2. **Edge Cases:**
   - Empty input
   - Very long text (overflow)
   - Special characters (emoji, unicode)
   - Fast typing (race conditions)
   - Multiple inputs on same page

3. **Keyboard Shortcuts:**
   - Cmd+A (select all)
   - Cmd+C (copy)
   - Cmd+V (paste)
   - Cmd+X (cut)
   - Arrow keys
   - Home/End

## Next Steps

For the `gpui-ui` project:

1. **Keep Current Input as "Display Only"**
   - Document it clearly
   - Use it for read-only displays
   - Rename to `InputDisplay` maybe?

2. **Create New `TextInput` Component**
   - Implement Phase 1 (basic typing)
   - Add to Phase 2 component list
   - Update todo app when ready

3. **Document Limitations**
   - Be clear about what works and doesn't
   - Provide workarounds (like "Add Random Todo")
   - Show users where to contribute

## Conclusion

Implementing a proper text input in GPUI is **non-trivial** but **definitely possible**. The complexity comes from managing:
- Focus state
- Keyboard events
- Text editing operations
- Cursor/selection rendering

Start with **Approach 1** (basic single-line input) to get something working, then iterate based on user feedback. Don't try to implement everything at once - build incrementally and test frequently.

The good news: Once you have a working text input, you can reuse it across all your GPUI applications!

## Want to Contribute?

If you implement a working text input component following this guide, please:
1. Open a PR to gpui-ui
2. Include tests and examples
3. Document any GPUI quirks you discovered
4. Share your learnings with the community!

Happy coding! 🚀

