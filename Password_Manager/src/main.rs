mod password_generator;
mod client;
mod clipboard;

fn main() -> iced::Result {
    client::app::run()
}
