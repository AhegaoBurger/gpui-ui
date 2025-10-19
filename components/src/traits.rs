use gpui::{App, ClickEvent, Window};

/// A trait for elements that can be clicked. Enables the use of the `on_click` method.
pub trait Clickable {
    /// Sets the click handler that will fire whenever the element is clicked.
    fn on_click(self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self;
}

/// A trait for elements that can be disabled.
pub trait Disableable {
    /// Sets whether the element is disabled.
    fn disabled(self, disabled: bool) -> Self;
}

/// A trait for elements that can be toggled.
pub trait Toggleable {
    /// Sets the toggle state of the element.
    fn toggle_state(self, state: ToggleState) -> Self;
}

/// Represents the selection status of a toggleable element.
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ToggleState {
    /// The element is not selected.
    #[default]
    Unselected,
    /// The selection state of the element is indeterminate.
    Indeterminate,
    /// The element is selected.
    Selected,
}

impl ToggleState {
    /// Returns the inverse of the current selection status.
    ///
    /// Indeterminate states become selected if inverted.
    pub fn inverse(&self) -> Self {
        match self {
            Self::Unselected | Self::Indeterminate => Self::Selected,
            Self::Selected => Self::Unselected,
        }
    }

    /// Returns whether this toggle state is selected
    pub fn selected(&self) -> bool {
        matches!(self, ToggleState::Selected)
    }
}

impl From<bool> for ToggleState {
    fn from(selected: bool) -> Self {
        if selected {
            Self::Selected
        } else {
            Self::Unselected
        }
    }
}

impl From<Option<bool>> for ToggleState {
    fn from(selected: Option<bool>) -> Self {
        match selected {
            Some(true) => Self::Selected,
            Some(false) => Self::Unselected,
            None => Self::Indeterminate,
        }
    }
}

