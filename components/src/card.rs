use crate::prelude::*;

/// Card variant determines the visual style
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CardVariant {
    Elevated,
    Outlined,
    Filled,
}

/// Card header component
pub struct CardHeader {
    title: Option<SharedString>,
    description: Option<SharedString>,
}

impl CardHeader {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl Default for CardHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for CardHeader {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let mut header = div()
            .flex()
            .flex_col()
            .gap_1()
            .p_6();

        if let Some(title) = self.title {
            header = header.child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x0f172a))
                    .child(title)
            );
        }

        if let Some(description) = self.description {
            header = header.child(
                div()
                    .text_sm()
                    .text_color(rgb(0x64748b))
                    .child(description)
            );
        }

        header
    }
}

/// Card content component
pub struct CardContent {
    children: Vec<AnyElement>,
}

impl CardContent {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children.extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl Default for CardContent {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for CardContent {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        div()
            .p_6()
            .pt_0()
            .children(self.children)
    }
}

/// Card footer component
pub struct CardFooter {
    children: Vec<AnyElement>,
}

impl CardFooter {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children.extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl Default for CardFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for CardFooter {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        div()
            .flex()
            .items_center()
            .gap_2()
            .p_6()
            .pt_0()
            .children(self.children)
    }
}

/// A card container component
pub struct Card {
    variant: CardVariant,
    children: Vec<AnyElement>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            variant: CardVariant::Outlined,
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn header(self, header: CardHeader) -> Self {
        self.child(header)
    }

    pub fn content(self, content: CardContent) -> Self {
        self.child(content)
    }

    pub fn footer(self, footer: CardFooter) -> Self {
        self.child(footer)
    }

    fn get_styles(&self) -> (Rgba, Option<Rgba>, Option<Hsla>) {
        match self.variant {
            CardVariant::Elevated => (
                rgb(0xffffff),
                None,
                Some(hsla(0.0, 0.0, 0.0, 0.1)),
            ),
            CardVariant::Outlined => (
                rgb(0xffffff),
                Some(rgb(0xe2e8f0)),
                None,
            ),
            CardVariant::Filled => (
                rgb(0xf8fafc),
                None,
                None,
            ),
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for Card {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let (bg_color, border_color, shadow) = self.get_styles();

        let mut card = div()
            .flex()
            .flex_col()
            .bg(bg_color)
            .rounded(px(8.0))
            .overflow_hidden()
            .children(self.children);

        if let Some(border) = border_color {
            card = card.border_1().border_color(border);
        }

        if let Some(_shadow_color) = shadow {
            // TODO: Add shadow support when GPUI shadow API is clarified
            // For now, using border to approximate elevated effect
            card = card.border_1().border_color(rgb(0xe5e7eb));
        }

        card
    }
}

