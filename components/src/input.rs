use crate::prelude::*;
use gpui::prelude::*;

/// Input type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputType {
    Text,
    Password,
    Email,
    Number,
    Search,
}

/// Input size options
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputSize {
    Small,
    Medium,
    Large,
}

/// Input variant (visual state)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputVariant {
    Default,
    Error,
    Success,
}

/// A text input component
pub struct Input {
    input_type: InputType,
    size: InputSize,
    variant: InputVariant,
    placeholder: Option<SharedString>,
    value: SharedString,
    label: Option<SharedString>,
    error: Option<SharedString>,
    disabled: bool,
    required: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            input_type: InputType::Text,
            size: InputSize::Medium,
            variant: InputVariant::Default,
            placeholder: None,
            value: "".into(),
            label: None,
            error: None,
            disabled: false,
            required: false,
        }
    }

    pub fn input_type(mut self, input_type: InputType) -> Self {
        self.input_type = input_type;
        self
    }

    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.error = Some(error.into());
        self.variant = InputVariant::Error;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    fn get_padding(&self) -> Pixels {
        match self.size {
            InputSize::Small => px(8.0),
            InputSize::Medium => px(10.0),
            InputSize::Large => px(12.0),
        }
    }

    fn get_text_size(&self) -> Pixels {
        match self.size {
            InputSize::Small => px(13.0),
            InputSize::Medium => px(14.0),
            InputSize::Large => px(16.0),
        }
    }

    fn get_border_color(&self) -> Rgba {
        if self.disabled {
            return rgb(0xe2e8f0);
        }

        match self.variant {
            InputVariant::Default => rgb(0xd1d5db),
            InputVariant::Error => rgb(0xef4444),
            InputVariant::Success => rgb(0x22c55e),
        }
    }

    fn get_background_color(&self) -> Rgba {
        if self.disabled {
            rgb(0xf1f5f9)
        } else {
            rgb(0xffffff)
        }
    }

    fn get_text_color(&self) -> Rgba {
        if self.disabled {
            rgb(0x94a3b8)
        } else {
            rgb(0x0f172a)
        }
    }

    fn get_placeholder_text(&self) -> String {
        if let Some(placeholder) = &self.placeholder {
            placeholder.to_string()
        } else {
            match self.input_type {
                InputType::Text => "Enter text...".to_string(),
                InputType::Password => "Enter password...".to_string(),
                InputType::Email => "Enter email...".to_string(),
                InputType::Number => "Enter number...".to_string(),
                InputType::Search => "Search...".to_string(),
            }
        }
    }

    fn render_value_or_placeholder(&self) -> Div {
        if self.value.is_empty() {
            div()
                .text_color(rgb(0x94a3b8))
                .child(self.get_placeholder_text())
        } else {
            let display_value = if self.input_type == InputType::Password {
                "•".repeat(self.value.len())
            } else {
                self.value.to_string()
            };
            
            div()
                .text_color(self.get_text_color())
                .child(display_value)
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for Input {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let padding = self.get_padding();
        let border_color = self.get_border_color();
        let bg_color = self.get_background_color();

        let input_field = div()
            .flex()
            .items_center()
            .w_full()
            .px(padding)
            .py(padding)
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .rounded(px(6.0))
            .text_size(self.get_text_size())
            .when(!self.disabled, |div| {
                div.cursor_text()
                    .hover(|style| style.border_color(rgb(0x94a3b8)))
            })
            .child(self.render_value_or_placeholder());

        let mut container = div()
            .flex()
            .flex_col()
            .gap_1()
            .w_full();

        // Label
        if let Some(label) = self.label {
            container = container.child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x0f172a))
                            .child(label)
                    )
                    .when(self.required, |d| {
                        d.child(
                            div()
                                .text_color(rgb(0xef4444))
                                .child("*")
                        )
                    })
            );
        }

        container = container.child(input_field);

        // Error message
        if let Some(error) = self.error {
            container = container.child(
                div()
                    .text_xs()
                    .text_color(rgb(0xef4444))
                    .child(error)
            );
        }

        container
    }
}

