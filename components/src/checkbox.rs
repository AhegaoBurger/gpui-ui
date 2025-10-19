use crate::prelude::*;
use gpui::prelude::*;

/// Checkbox state
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CheckboxState {
    Unchecked,
    Checked,
    Indeterminate,
}

/// Checkbox size options
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CheckboxSize {
    Small,
    Medium,
    Large,
}

/// A checkbox input component
pub struct Checkbox {
    state: CheckboxState,
    size: CheckboxSize,
    label: Option<SharedString>,
    disabled: bool,
}

impl Checkbox {
    pub fn new() -> Self {
        Self {
            state: CheckboxState::Unchecked,
            size: CheckboxSize::Medium,
            label: None,
            disabled: false,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.state = if checked {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        };
        self
    }

    pub fn state(mut self, state: CheckboxState) -> Self {
        self.state = state;
        self
    }

    pub fn size(mut self, size: CheckboxSize) -> Self {
        self.size = size;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    fn get_box_size(&self) -> Pixels {
        match self.size {
            CheckboxSize::Small => px(16.0),
            CheckboxSize::Medium => px(20.0),
            CheckboxSize::Large => px(24.0),
        }
    }

    fn get_icon_size(&self) -> Pixels {
        match self.size {
            CheckboxSize::Small => px(10.0),
            CheckboxSize::Medium => px(12.0),
            CheckboxSize::Large => px(14.0),
        }
    }

    fn get_background_color(&self) -> Rgba {
        if self.disabled {
            return rgb(0xf1f5f9);
        }

        match self.state {
            CheckboxState::Unchecked => rgb(0xffffff),
            CheckboxState::Checked | CheckboxState::Indeterminate => rgb(0x3b82f6),
        }
    }

    fn get_border_color(&self) -> Rgba {
        if self.disabled {
            return rgb(0xe2e8f0);
        }

        match self.state {
            CheckboxState::Unchecked => rgb(0xd1d5db),
            CheckboxState::Checked | CheckboxState::Indeterminate => rgb(0x3b82f6),
        }
    }

    fn render_icon(&self) -> Option<Div> {
        if self.disabled {
            return None;
        }

        let icon_size = self.get_icon_size();

        match self.state {
            CheckboxState::Unchecked => None,
            CheckboxState::Checked => Some(
                // Checkmark
                div()
                    .absolute()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size_full()
                    .text_color(rgb(0xffffff))
                    .font_weight(FontWeight::BOLD)
                    .text_size(icon_size)
                    .child("✓")
            ),
            CheckboxState::Indeterminate => Some(
                // Dash/minus
                div()
                    .absolute()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size_full()
                    .text_color(rgb(0xffffff))
                    .font_weight(FontWeight::BOLD)
                    .text_size(icon_size)
                    .child("−")
            ),
        }
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for Checkbox {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let box_size = self.get_box_size();
        let bg_color = self.get_background_color();
        let border_color = self.get_border_color();

        let checkbox_box = div()
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .size(box_size)
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .rounded(px(4.0))
            .when(!self.disabled, |div| {
                div.cursor_pointer()
                    .hover(|style| style.border_color(rgb(0x94a3b8)))
            })
            .when_some(self.render_icon(), |div, icon| div.child(icon));

        let mut container = div()
            .flex()
            .items_center()
            .gap_2()
            .child(checkbox_box);

        if let Some(label) = self.label {
            container = container.child(
                div()
                    .text_sm()
                    .text_color(if self.disabled {
                        rgb(0x94a3b8)
                    } else {
                        rgb(0x0f172a)
                    })
                    .when(!self.disabled, |div| div.cursor_pointer())
                    .child(label)
            );
        }

        container
    }
}

