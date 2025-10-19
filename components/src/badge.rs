use crate::prelude::*;

/// Badge variant determines the visual style
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BadgeVariant {
    Default,
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
    Outline,
}

/// Badge size options
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BadgeSize {
    Small,
    Medium,
    Large,
}

/// A badge component for labels, tags, and status indicators
#[derive(IntoElement)]
pub struct Badge {
    variant: BadgeVariant,
    size: BadgeSize,
    label: SharedString,
    dot: bool,
}

impl Badge {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            variant: BadgeVariant::Default,
            size: BadgeSize::Medium,
            label: label.into(),
            dot: false,
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: BadgeSize) -> Self {
        self.size = size;
        self
    }

    pub fn dot(mut self, dot: bool) -> Self {
        self.dot = dot;
        self
    }

    fn get_padding(&self) -> (Pixels, Pixels) {
        match self.size {
            BadgeSize::Small => (px(4.0), px(2.0)),
            BadgeSize::Medium => (px(6.0), px(3.0)),
            BadgeSize::Large => (px(8.0), px(4.0)),
        }
    }

    fn get_text_size(&self) -> Pixels {
        match self.size {
            BadgeSize::Small => px(11.0),
            BadgeSize::Medium => px(12.0),
            BadgeSize::Large => px(14.0),
        }
    }

    fn get_background_color(&self) -> Rgba {
        match self.variant {
            BadgeVariant::Default => rgb(0xf1f5f9),      // muted
            BadgeVariant::Primary => rgb(0x3b82f6),      // primary
            BadgeVariant::Secondary => rgb(0x64748b),    // secondary
            BadgeVariant::Success => rgb(0x22c55e),      // green
            BadgeVariant::Warning => rgb(0xf59e0b),      // amber
            BadgeVariant::Error => rgb(0xef4444),        // red
            BadgeVariant::Outline => rgb(0xffffff),      // white
        }
    }

    fn get_text_color(&self) -> Rgba {
        match self.variant {
            BadgeVariant::Default => rgb(0x0f172a),      // dark
            BadgeVariant::Primary | BadgeVariant::Secondary | 
            BadgeVariant::Success | BadgeVariant::Error => rgb(0xffffff),
            BadgeVariant::Warning => rgb(0x78350f),      // dark amber
            BadgeVariant::Outline => rgb(0x0f172a),      // dark
        }
    }

    fn get_border_color(&self) -> Option<Rgba> {
        match self.variant {
            BadgeVariant::Outline => Some(rgb(0xe2e8f0)),
            _ => None,
        }
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (px_padding, py_padding) = self.get_padding();
        let bg_color = self.get_background_color();
        let text_color = self.get_text_color();
        let border_color = self.get_border_color();

        let mut badge = div()
            .flex()
            .items_center()
            .gap_1()
            .px(px_padding)
            .py(py_padding)
            .bg(bg_color)
            .text_color(text_color)
            .text_size(self.get_text_size())
            .rounded(px(9999.0)) // fully rounded
            .font_weight(FontWeight::MEDIUM);

        if let Some(border) = border_color {
            badge = badge.border_1().border_color(border);
        }

        if self.dot {
            badge = badge.child(
                div()
                    .size(px(6.0))
                    .rounded(px(9999.0))
                    .bg(text_color)
            );
        }

        badge = badge.child(self.label.clone());

        badge
    }
}
