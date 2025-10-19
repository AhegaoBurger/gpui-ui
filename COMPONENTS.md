# GPUI-UI Component Library

This document outlines all planned components for the GPUI-UI library, inspired by shadcn/ui but adapted for GPUI.

## Component Status Legend

- ✅ **Implemented** - Component is fully functional and tested
- 🚧 **In Progress** - Component is being developed
- 📋 **Planned** - Component is on the roadmap
- 🔮 **Future** - Component for later consideration

---

## Core Components

### Form Components

#### ✅ Button
**Status:** Implemented  
**Path:** `components/src/button.rs`  
**Variants:** Default, Destructive, Outline, Ghost, Link  
**Sizes:** Small, Medium, Large  

**Features:**
- Multiple visual variants
- Size options
- Disabled state
- Hover effects
- Accessible

**Usage:**
```rust
Button::new("Click me")
    .variant(ButtonVariant::Default)
    .size(ButtonSize::Medium)
```

---

#### 📋 Input
**Status:** Planned - Priority 1  
**Path:** `components/src/input.rs`  

**Variants:**
- Text input
- Password input
- Email input
- Number input
- Search input
- Textarea

**Features:**
- Placeholder text
- Label support
- Error states
- Disabled state
- Icon support (leading/trailing)
- Character counter
- Validation states (valid/invalid/warning)
- Clear button

**API Design:**
```rust
Input::new()
    .placeholder("Enter your email...")
    .input_type(InputType::Email)
    .label("Email Address")
    .error("Invalid email format")
    .required(true)
    .on_change(|value| { /* handle change */ })
```

---

#### 📋 Checkbox
**Status:** Planned - Priority 1  
**Path:** `components/src/checkbox.rs`  

**Features:**
- Checked/unchecked/indeterminate states
- Label support
- Disabled state
- Custom check icon
- Size options

**API Design:**
```rust
Checkbox::new()
    .label("Accept terms and conditions")
    .checked(false)
    .on_change(|checked| { /* handle change */ })
```

---

#### 📋 Radio Group
**Status:** Planned - Priority 2  
**Path:** `components/src/radio.rs`  

**Features:**
- Radio button group
- Single selection
- Disabled options
- Horizontal/vertical layout

**API Design:**
```rust
RadioGroup::new()
    .options(vec![
        RadioOption::new("option1", "Option 1"),
        RadioOption::new("option2", "Option 2"),
    ])
    .selected("option1")
    .on_change(|value| { /* handle change */ })
```

---

#### 📋 Select
**Status:** Planned - Priority 2  
**Path:** `components/src/select.rs`  

**Features:**
- Single/multiple selection
- Search/filter
- Keyboard navigation
- Custom option rendering
- Grouped options
- Placeholder

**API Design:**
```rust
Select::new()
    .placeholder("Select an option...")
    .options(vec!["Option 1", "Option 2", "Option 3"])
    .searchable(true)
    .on_change(|value| { /* handle change */ })
```

---

#### 📋 Switch / Toggle
**Status:** Planned - Priority 2  
**Path:** `components/src/switch.rs`  

**Features:**
- On/off states
- Label support
- Disabled state
- Animated transition

**API Design:**
```rust
Switch::new()
    .label("Enable notifications")
    .checked(true)
    .on_change(|checked| { /* handle change */ })
```

---

#### 📋 Slider
**Status:** Planned - Priority 3  
**Path:** `components/src/slider.rs`  

**Features:**
- Single/range selection
- Step control
- Min/max values
- Value display
- Vertical/horizontal

**API Design:**
```rust
Slider::new()
    .min(0.0)
    .max(100.0)
    .step(5.0)
    .value(50.0)
    .on_change(|value| { /* handle change */ })
```

---

### Layout Components

#### 📋 Card
**Status:** Planned - Priority 1  
**Path:** `components/src/card.rs`  

**Features:**
- Header section
- Content section
- Footer section
- Variants (elevated, outlined)

**API Design:**
```rust
Card::new()
    .header(CardHeader::new("Card Title").description("Subtitle"))
    .content(CardContent::new().child(/* content */))
    .footer(CardFooter::new().child(/* footer */))
```

---

#### 📋 Separator / Divider
**Status:** Planned - Priority 2  
**Path:** `components/src/separator.rs`  

**Features:**
- Horizontal/vertical
- Label support
- Different styles (solid, dashed)

**API Design:**
```rust
Separator::new()
    .orientation(Orientation::Horizontal)
    .label("OR")
```

---

#### 📋 Accordion
**Status:** Planned - Priority 3  
**Path:** `components/src/accordion.rs`  

**Features:**
- Expandable sections
- Single/multiple expansion
- Animated transitions
- Custom icons

**API Design:**
```rust
Accordion::new()
    .items(vec![
        AccordionItem::new("item1", "Title 1", content1),
        AccordionItem::new("item2", "Title 2", content2),
    ])
    .allow_multiple(false)
```

---

#### 📋 Tabs
**Status:** Planned - Priority 2  
**Path:** `components/src/tabs.rs`  

**Features:**
- Multiple tab panels
- Keyboard navigation
- Disabled tabs
- Custom styling

**API Design:**
```rust
Tabs::new()
    .tabs(vec![
        Tab::new("tab1", "Tab 1", content1),
        Tab::new("tab2", "Tab 2", content2),
    ])
    .default_tab("tab1")
```

---

### Feedback Components

#### 📋 Alert / Banner
**Status:** Planned - Priority 2  
**Path:** `components/src/alert.rs`  

**Features:**
- Variants (info, success, warning, error)
- Dismissible
- Icon support
- Title and description

**API Design:**
```rust
Alert::new()
    .variant(AlertVariant::Warning)
    .title("Warning")
    .description("This action cannot be undone")
    .dismissible(true)
```

---

#### 📋 Toast / Notification
**Status:** Planned - Priority 2  
**Path:** `components/src/toast.rs`  

**Features:**
- Auto-dismiss with timer
- Multiple positions (top-right, bottom-left, etc.)
- Queue management
- Variants (info, success, warning, error)
- Action button support

**API Design:**
```rust
Toast::show(cx, ToastOptions {
    title: "Success".into(),
    description: "Item saved successfully".into(),
    variant: ToastVariant::Success,
    duration: Duration::from_secs(3),
    position: ToastPosition::TopRight,
})
```

---

#### 📋 Progress Bar
**Status:** Planned - Priority 2  
**Path:** `components/src/progress.rs`  

**Features:**
- Determinate/indeterminate
- Size variants
- Color variants
- Label support

**API Design:**
```rust
Progress::new()
    .value(45.0)
    .max(100.0)
    .label("45%")
```

---

#### 📋 Spinner / Loading
**Status:** Planned - Priority 2  
**Path:** `components/src/spinner.rs`  

**Features:**
- Size variants
- Color variants
- With/without label

**API Design:**
```rust
Spinner::new()
    .size(SpinnerSize::Medium)
    .label("Loading...")
```

---

#### 📋 Skeleton
**Status:** Planned - Priority 3  
**Path:** `components/src/skeleton.rs`  

**Features:**
- Various shapes (text, circle, rectangle)
- Animated pulse
- Custom dimensions

**API Design:**
```rust
Skeleton::new()
    .variant(SkeletonVariant::Text)
    .width(px(200.0))
    .animated(true)
```

---

### Overlay Components

#### 📋 Dialog / Modal
**Status:** Planned - Priority 1  
**Path:** `components/src/dialog.rs`  

**Features:**
- Backdrop overlay
- Close on backdrop click
- Close button
- Header, content, footer sections
- Keyboard handling (ESC to close)
- Focus trap
- Size variants

**API Design:**
```rust
Dialog::new()
    .title("Confirm Action")
    .content(DialogContent::new().child(/* content */))
    .footer(DialogFooter::new().child(/* buttons */))
    .close_on_backdrop(true)
    .on_close(|| { /* handle close */ })
```

---

#### 📋 Popover
**Status:** Planned - Priority 2  
**Path:** `components/src/popover.rs`  

**Features:**
- Position relative to trigger
- Multiple placement options
- Arrow indicator
- Click outside to close

**API Design:**
```rust
Popover::new()
    .trigger(Button::new("Show Popover"))
    .content(/* popover content */)
    .placement(Placement::Bottom)
```

---

#### 📋 Tooltip
**Status:** Planned - Priority 2  
**Path:** `components/src/tooltip.rs`  

**Features:**
- Hover/focus triggered
- Multiple placements
- Delay control
- Arrow indicator

**API Design:**
```rust
Tooltip::new()
    .child(Button::new("Hover me"))
    .content("This is a tooltip")
    .placement(Placement::Top)
    .delay(Duration::from_millis(500))
```

---

#### 📋 Dropdown Menu
**Status:** Planned - Priority 2  
**Path:** `components/src/dropdown.rs`  

**Features:**
- Menu items
- Separators
- Submenus
- Icons
- Keyboard navigation
- Checkable items

**API Design:**
```rust
DropdownMenu::new()
    .trigger(Button::new("Menu"))
    .items(vec![
        MenuItem::new("item1", "Action 1"),
        MenuItem::separator(),
        MenuItem::new("item2", "Action 2"),
    ])
```

---

#### 📋 Context Menu
**Status:** Planned - Priority 3  
**Path:** `components/src/context_menu.rs`  

**Features:**
- Right-click triggered
- Similar to dropdown menu
- Submenus

**API Design:**
```rust
ContextMenu::new()
    .trigger(/* element to right-click */)
    .items(vec![/* menu items */])
```

---

### Display Components

#### 📋 Badge
**Status:** Planned - Priority 1  
**Path:** `components/src/badge.rs`  

**Features:**
- Variants (default, primary, secondary, success, warning, error)
- Sizes
- Dismissible option
- Dot variant

**API Design:**
```rust
Badge::new("New")
    .variant(BadgeVariant::Primary)
    .size(BadgeSize::Small)
```

---

#### 📋 Avatar
**Status:** Planned - Priority 2  
**Path:** `components/src/avatar.rs`  

**Features:**
- Image support
- Fallback initials
- Size variants
- Status indicator (online/offline)
- Group support (overlapping avatars)

**API Design:**
```rust
Avatar::new()
    .image("path/to/image.png")
    .fallback("JD")
    .size(AvatarSize::Medium)
    .status(AvatarStatus::Online)
```

---

#### 📋 Table
**Status:** Planned - Priority 3  
**Path:** `components/src/table.rs`  

**Features:**
- Header/body/footer
- Sortable columns
- Selectable rows
- Pagination
- Fixed header
- Responsive

**API Design:**
```rust
Table::new()
    .columns(vec![
        Column::new("name", "Name").sortable(true),
        Column::new("email", "Email"),
    ])
    .data(data)
    .on_sort(|column| { /* handle sort */ })
```

---

#### 📋 List
**Status:** Planned - Priority 2  
**Path:** `components/src/list.rs`  

**Features:**
- Ordered/unordered
- Custom item rendering
- Dividers
- Icons

**API Design:**
```rust
List::new()
    .items(vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
    ])
```

---

### Navigation Components

#### 📋 Breadcrumb
**Status:** Planned - Priority 3  
**Path:** `components/src/breadcrumb.rs`  

**Features:**
- Hierarchical navigation
- Custom separator
- Current page indicator
- Truncation for long paths

**API Design:**
```rust
Breadcrumb::new()
    .items(vec![
        BreadcrumbItem::new("Home", "/"),
        BreadcrumbItem::new("Products", "/products"),
        BreadcrumbItem::new("Item", "/products/item").current(true),
    ])
```

---

#### 📋 Pagination
**Status:** Planned - Priority 3  
**Path:** `components/src/pagination.rs`  

**Features:**
- Page numbers
- Previous/next buttons
- Jump to page
- Items per page selector

**API Design:**
```rust
Pagination::new()
    .total_pages(10)
    .current_page(1)
    .on_page_change(|page| { /* handle change */ })
```

---

## Implementation Priority

### Phase 1 (MVP) - Core Essentials
**Goal:** Basic form and layout components

1. ✅ Button (Done)
2. 📋 Input
3. 📋 Checkbox
4. 📋 Card
5. 📋 Badge
6. 📋 Dialog/Modal

### Phase 2 - Forms & Feedback
**Goal:** Complete form handling and user feedback

7. 📋 Select
8. 📋 Radio Group
9. 📋 Switch
10. 📋 Alert
11. 📋 Toast
12. 📋 Progress

### Phase 3 - Advanced Interactions
**Goal:** Rich interactive components

13. 📋 Tabs
14. 📋 Accordion
15. 📋 Dropdown Menu
16. 📋 Popover
17. 📋 Tooltip
18. 📋 Spinner

### Phase 4 - Display & Navigation
**Goal:** Data display and navigation

19. 📋 Avatar
20. 📋 List
21. 📋 Separator
22. 📋 Breadcrumb
23. 📋 Pagination

### Phase 5 - Advanced Components
**Goal:** Complex components for rich applications

24. 📋 Table
25. 📋 Slider
26. 📋 Context Menu
27. 📋 Skeleton

## Design Principles

Each component should follow these principles:

1. **Composable** - Components can be nested and combined
2. **Accessible** - Keyboard navigation and screen reader support where applicable
3. **Themeable** - Use design tokens from config (colors, spacing, radius)
4. **Flexible** - Builder pattern with sensible defaults
5. **Type-safe** - Use Rust's type system to prevent misuse
6. **Documented** - Clear examples and API documentation
7. **Tested** - Unit tests and example usage

## Component Template

Each component should have:

```rust
// File: components/src/component_name.rs

use crate::prelude::*;

/// Component variant
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComponentVariant {
    Default,
    // ... other variants
}

/// Component size
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComponentSize {
    Small,
    Medium,
    Large,
}

/// Main component struct
pub struct Component {
    variant: ComponentVariant,
    size: ComponentSize,
    // ... other fields
}

impl Component {
    pub fn new(/* required params */) -> Self {
        Self {
            variant: ComponentVariant::Default,
            size: ComponentSize::Medium,
            // ... defaults
        }
    }

    // Builder methods
    pub fn variant(mut self, variant: ComponentVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    // Helper methods for styling
    fn get_styles(&self) -> ComponentStyles {
        // ... style computation
    }
}

impl IntoElement for Component {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        div()
            // ... GPUI styling
    }
}
```

## Component Metadata

Each component needs metadata for the CLI:

```json
{
  "name": "button",
  "version": "0.1.0",
  "description": "A customizable button component",
  "files": [
    "button.rs"
  ],
  "dependencies": [],
  "tags": ["form", "interactive"],
  "examples": [
    "examples/button_demo.rs"
  ]
}
```

## Next Steps

1. Review and prioritize Phase 1 components
2. Create detailed API specs for next 3 components (Input, Checkbox, Card)
3. Implement component registry system in CLI
4. Set up component testing framework
5. Create component showcase/documentation site

---

**Last Updated:** 2025-10-19  
**Contributors:** AI Assistant, Project Team

