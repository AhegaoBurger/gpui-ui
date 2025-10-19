use crate::prelude::*;

/// Checkbox size options
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CheckboxSize {
    Small,
    Medium,
    Large,
}

/// A checkbox input component
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    state: ToggleState,
    size: CheckboxSize,
    label: Option<SharedString>,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ToggleState, &mut Window, &mut App) + 'static>>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>, state: ToggleState) -> Self {
        Self {
            id: id.into(),
            state,
            size: CheckboxSize::Medium,
            label: None,
            disabled: false,
            on_click: None,
        }
    }

    pub fn checked(id: impl Into<ElementId>, checked: bool) -> Self {
        Self::new(id, ToggleState::from(checked))
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

    pub fn on_click(
        mut self,
        handler: impl Fn(&ToggleState, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
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
            ToggleState::Unselected => rgb(0xffffff),
            ToggleState::Selected | ToggleState::Indeterminate => rgb(0x3b82f6),
        }
    }

    fn get_border_color(&self) -> Rgba {
        if self.disabled {
            return rgb(0xe2e8f0);
        }

        match self.state {
            ToggleState::Unselected => rgb(0xd1d5db),
            ToggleState::Selected | ToggleState::Indeterminate => rgb(0x3b82f6),
        }
    }

    fn render_icon(&self) -> Option<Div> {
        if self.disabled {
            return None;
        }

        let icon_size = self.get_icon_size();

        match self.state {
            ToggleState::Unselected => None,
            ToggleState::Selected => Some(
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
            ToggleState::Indeterminate => Some(
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

impl Toggleable for Checkbox {
    fn toggle_state(mut self, state: ToggleState) -> Self {
        self.state = state;
        self
    }
}

impl Disableable for Checkbox {
    fn disabled(self, disabled: bool) -> Self {
        Self { disabled, ..self }
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
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
            .when(self.disabled, |div| div.cursor_not_allowed())
            .when_some(self.render_icon(), |div, icon| div.child(icon));

        let mut container = div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_2()
            .child(checkbox_box);

        // Add click handler to the whole container
        if !self.disabled {
            if let Some(handler) = self.on_click {
                let new_state = self.state.inverse();
                container = container.on_click(move |_event, window, cx| {
                    handler(&new_state, window, cx);
                });
            }
        }

        // Add label if present
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
