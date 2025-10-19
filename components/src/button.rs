use crate::prelude::*;

/// Button variant determines the visual style
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Ghost,
    Link,
}

/// Button size options
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

/// A customizable button component
pub struct Button {
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    label: SharedString,
}

impl Button {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            variant: ButtonVariant::Default,
            size: ButtonSize::Medium,
            disabled: false,
            label: label.into(),
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

    fn get_padding(&self) -> Pixels {
        match self.size {
            ButtonSize::Small => px(8.0),
            ButtonSize::Medium => px(12.0),
            ButtonSize::Large => px(16.0),
        }
    }

    fn get_background_color(&self) -> Rgba {
        if self.disabled {
            return rgb(0xf1f5f9); // muted
        }

        match self.variant {
            ButtonVariant::Default => rgb(0x3b82f6),      // primary
            ButtonVariant::Destructive => rgb(0xef4444),  // destructive
            ButtonVariant::Outline => rgb(0xffffff),      // white
            ButtonVariant::Ghost => rgb(0x00000000),      // transparent
            ButtonVariant::Link => rgb(0x00000000),       // transparent
        }
    }

    fn get_text_color(&self) -> Rgba {
        if self.disabled {
            return rgb(0x94a3b8); // gray
        }

        match self.variant {
            ButtonVariant::Default | ButtonVariant::Destructive => rgb(0xffffff),
            ButtonVariant::Outline | ButtonVariant::Ghost => rgb(0x0f172a),
            ButtonVariant::Link => rgb(0x3b82f6),
        }
    }

    fn get_border_color(&self) -> Option<Rgba> {
        match self.variant {
            ButtonVariant::Outline => Some(rgb(0xe2e8f0)),
            _ => None,
        }
    }
}

impl IntoElement for Button {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let padding = self.get_padding();
        let bg_color = self.get_background_color();
        let text_color = self.get_text_color();
        let border_color = self.get_border_color();

        let mut button = div()
            .flex()
            .items_center()
            .justify_center()
            .px(padding * 2.0)
            .py(padding)
            .bg(bg_color)
            .text_color(text_color)
            .rounded(px(4.0))
            .cursor_pointer()
            .child(self.label.clone());

        if let Some(border) = border_color {
            button = button.border_1().border_color(border);
        }

        if !self.disabled {
            button = button.hover(|style| {
                style.opacity(0.9)
            });
        }

        button
    }
}
