mod client;
mod password_generator;

fn main() -> iced::Result {
    client::app::run()
}
