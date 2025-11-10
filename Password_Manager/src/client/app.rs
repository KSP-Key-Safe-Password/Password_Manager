use crate::client::framework::Controller;
use crate::client::pages::generator::{GeneratorController, Message as GeneratorMessage};
use iced::{Element, Task};

pub struct PasswordManagerApp {
    primary_controller: Box<dyn Controller>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Generator(GeneratorMessage),
}

impl PasswordManagerApp {
    fn new() -> (Self, Task<Message>) {
        let (controller, task) = GeneratorController::bootstrap();
        (
            Self {
                primary_controller: controller,
            },
            task,
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        if self.primary_controller.accepts(&message) {
            self.primary_controller.update(message)
        } else {
            #[cfg(debug_assertions)]
            {
                eprintln!(
                    "Controller '{}' ignored message: {:?}",
                    self.primary_controller.name(),
                    message
                );
            }
            Task::none()
        }
    }

    fn view(&self) -> Element<'_, Message> {
        self.primary_controller.view()
    }
}

pub fn run() -> iced::Result {
    iced::application(
        "Password Manager",
        PasswordManagerApp::update,
        PasswordManagerApp::view,
    )
    .run_with(PasswordManagerApp::new)
}
