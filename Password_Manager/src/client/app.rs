use crate::client::pages::generator::{
    GeneratorPage,
    Message as GeneratorMessage,
};
use iced::{Element, Task};

pub struct PasswordManagerApp {
    generator: GeneratorPage,
}

#[derive(Debug, Clone)]
pub enum Message {
    Generator(GeneratorMessage),
}

impl PasswordManagerApp {
    fn new() -> (Self, Task<Message>) {
        let (generator, task) = GeneratorPage::new();
        (
            Self { generator },
            task.map(Message::Generator),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Generator(msg) => self
                .generator
                .update(msg)
                .map(Message::Generator),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        self.generator.view().map(Message::Generator)
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
