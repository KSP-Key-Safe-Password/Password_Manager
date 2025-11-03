use iced::{Element, Task};

pub struct PasswordManager {

}

#[derive(Debug, Clone)]
pub enum Message {

}

impl PasswordManager {
    fn new() -> (Self, Task<Message>) {
        (
            Self {},
            Task::none(),
        )
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        iced::widget::text("Password Manager - GUI Coming Soon").into()
    }
}

pub fn run() -> iced::Result {
    iced::application("Password Manager", PasswordManager::update, PasswordManager::view)
        .run_with(PasswordManager::new)
}