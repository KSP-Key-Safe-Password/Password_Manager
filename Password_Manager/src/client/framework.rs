use iced::{Element, Task};

use crate::client::app::Message;

#[derive(Debug)]
pub struct ControllerCore {
    id: &'static str,
}

impl ControllerCore {
    pub fn new(id: &'static str) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &'static str {
        self.id
    }
}

#[derive(Debug)]
pub struct PageSurface {
    title: &'static str,
}

impl PageSurface {
    pub fn new(title: &'static str) -> Self {
        Self { title }
    }

    pub fn title(&self) -> &'static str {
        self.title
    }
}

/// Common interface for runtime-dispatchable controllers.
///
/// Each controller is expected to "inherit" [`ControllerCore`] (via the
/// `inherit` crate) and expose the shared metadata through [`Controller::core`].
pub trait Controller {
    fn core(&self) -> &ControllerCore;

    /// Returns a stable identifier for diagnostics or routing.
    fn name(&self) -> &'static str {
        self.core().id()
    }

    /// Determines whether the controller wants to handle the provided message.
    fn accepts(&self, message: &Message) -> bool;

    /// Handles the provided message and returns the next task.
    fn update(&mut self, message: Message) -> Task<Message>;

    /// Renders the UI for the controller.
    fn view(&self) -> Element<'_, Message>;
}
