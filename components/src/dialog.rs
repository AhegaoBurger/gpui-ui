use crate::prelude::*;

/// Dialog size options
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DialogSize {
    Small,
    Medium,
    Large,
    Full,
}

/// Dialog header component
pub struct DialogHeader {
    title: Option<SharedString>,
    description: Option<SharedString>,
}

impl DialogHeader {
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

impl Default for DialogHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for DialogHeader {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let mut header = div()
            .flex()
            .flex_col()
            .gap_1();

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

/// Dialog content component
pub struct DialogContent {
    children: Vec<AnyElement>,
}

impl DialogContent {
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

impl Default for DialogContent {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for DialogContent {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .py_4()
            .children(self.children)
    }
}

/// Dialog footer component
pub struct DialogFooter {
    children: Vec<AnyElement>,
}

impl DialogFooter {
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

impl Default for DialogFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for DialogFooter {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        div()
            .flex()
            .justify_end()
            .gap_2()
            .children(self.children)
    }
}

/// A dialog/modal overlay component
pub struct Dialog {
    size: DialogSize,
    open: bool,
    children: Vec<AnyElement>,
}

impl Dialog {
    pub fn new() -> Self {
        Self {
            size: DialogSize::Medium,
            open: true,
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: DialogSize) -> Self {
        self.size = size;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn header(self, header: DialogHeader) -> Self {
        self.child(header)
    }

    pub fn content(self, content: DialogContent) -> Self {
        self.child(content)
    }

    pub fn footer(self, footer: DialogFooter) -> Self {
        self.child(footer)
    }

    fn get_width(&self) -> Pixels {
        match self.size {
            DialogSize::Small => px(400.0),
            DialogSize::Medium => px(500.0),
            DialogSize::Large => px(700.0),
            DialogSize::Full => px(900.0),
        }
    }
}

impl Default for Dialog {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for Dialog {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        if !self.open {
            return div(); // Empty div when closed
        }

        let dialog_width = self.get_width();

        // Overlay backdrop
        div()
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5)) // Semi-transparent black backdrop
            .child(
                // Dialog content
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .w(dialog_width)
                    .max_h(px(600.0))
                    .p_6()
                    .bg(rgb(0xffffff))
                    .rounded(px(12.0))
                    .border_1()
                    .border_color(rgb(0xe5e7eb))
                    // TODO: Add shadow when GPUI shadow API is clarified
                    .children(self.children)
            )
    }
}

