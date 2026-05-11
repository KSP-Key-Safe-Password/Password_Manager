use crate::password_generator::{
    generate_password, validate_password_settings, PasswordSettings,
    MAX_PASSWORD_LENGTH, MIN_PASSWORD_LENGTH,
};
use arboard::Clipboard;
use iced::alignment;
use iced::widget::{
    button, checkbox, container, text, text_input, Column, Row,
};
use iced::{Element, Length, Task};

pub struct GeneratorPage {
    settings: PasswordSettings,
    generated_password: Option<String>,
    status_message: Option<String>,
    length_value: u16,
    length_input: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    LengthInputChanged(String),
    ToggleUppercase(bool),
    ToggleNumbers(bool),
    ToggleSpecial(bool),
    GeneratePressed,
    CopyPressed,
}

impl GeneratorPage {
    pub fn new() -> (Self, Task<Message>) {
        let settings = PasswordSettings::default();
        (
            Self {
                length_value: settings.length as u16,
                length_input: settings.length.to_string(),
                settings,
                generated_password: None,
                status_message: None,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LengthInputChanged(value) => {
                let filtered: String =
                    value.chars().filter(|ch| ch.is_ascii_digit()).collect();
                let mut status_message: Option<String> = None;

                if filtered.is_empty() {
                    self.length_input.clear();
                    self.status_message = if value.is_empty() {
                        Some("Please enter a password length.".into())
                    } else {
                        Some("Only digits are allowed for the password length.".into())
                    };
                    return Task::none();
                }

                if filtered != value {
                    status_message =
                        Some("Only digits are allowed for the password length.".into());
                }

                match filtered.parse::<u32>() {
                    Ok(parsed) => {
                        let min = MIN_PASSWORD_LENGTH as u32;
                        let max = MAX_PASSWORD_LENGTH as u32;

                        if parsed > max {
                            self.length_value = max as u16;
                            self.settings.length = max as usize;
                            self.length_input = max.to_string();
                            status_message = Some(format!(
                                "Please enter a number between {MIN_PASSWORD_LENGTH} and {}.",
                                MAX_PASSWORD_LENGTH
                            ));
                        } else {
                            self.length_input = filtered;

                            if parsed < min {
                                status_message = Some(format!(
                                    "Please enter a number between {MIN_PASSWORD_LENGTH} and {}.",
                                    MAX_PASSWORD_LENGTH
                                ));
                            } else {
                                self.length_value = parsed as u16;
                                self.settings.length = parsed as usize;
                            }
                        }
                    }
                    Err(_) => {
                        self.length_input = filtered;
                        status_message = Some("Please enter a valid number.".into());
                    }
                }

                self.status_message = status_message;
            }
            Message::ToggleUppercase(value) => {
                self.settings.include_uppercase = value;
                self.refresh_validation_status();
            }
            Message::ToggleNumbers(value) => {
                self.settings.include_numbers = value;
                self.refresh_validation_status();
            }
            Message::ToggleSpecial(value) => {
                self.settings.include_special_chars = value;
                self.refresh_validation_status();
            }
            Message::GeneratePressed => match generate_password(&self.settings) {
                Ok(password) => {
                    self.generated_password = Some(password);
                    self.status_message = Some("Generated password.".into());
                }
                Err(err) => {
                    self.generated_password = None;
                    self.status_message = Some(err.to_string());
                }
            },
            Message::CopyPressed => {
                if let Some(password) = &self.generated_password {
                    match Clipboard::new()
                        .and_then(|mut clipboard| clipboard.set_text(password.clone()))
                    {
                        Ok(()) => {
                            self.status_message =
                                Some("Password copied to clipboard.".into());
                        }
                        Err(err) => {
                            self.status_message =
                                Some(format!("Failed to copy password: {err}"));
                        }
                    }
                } else {
                    self.status_message = Some("No password to copy yet.".into());
                }
            }
        }

        Task::none()
    }

    fn refresh_validation_status(&mut self) {
        match validate_password_settings(&self.settings) {
            Ok(()) => self.status_message = None,
            Err(err) => self.status_message = Some(err.to_string()),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let header = text("Password Manager")
            .size(32)
            .align_x(alignment::Horizontal::Center);

        let length_controls = Column::new()
            .spacing(8)
            .width(Length::FillPortion(2))
            .push(text(format!(
                "Password length ({MIN_PASSWORD_LENGTH}-{MAX_PASSWORD_LENGTH})"
            )))
            .push(
                text_input("Length", &self.length_input)
                    .on_input(Message::LengthInputChanged),
            );

        let toggles = Column::new()
            .spacing(8)
            .width(Length::FillPortion(2))
            .push(
                checkbox("Include uppercase letters", self.settings.include_uppercase)
                    .on_toggle(Message::ToggleUppercase),
            )
            .push(
                checkbox("Include numbers", self.settings.include_numbers)
                    .on_toggle(Message::ToggleNumbers),
            )
            .push(
                checkbox(
                    "Include special characters",
                    self.settings.include_special_chars,
                )
                .on_toggle(Message::ToggleSpecial),
            );

        let mut copy_button = button("Copy to clipboard");
        if self.generated_password.is_some() {
            copy_button = copy_button.on_press(Message::CopyPressed);
        }

        let can_generate = self
            .length_input
            .parse::<u32>()
            .map(|value| {
                value >= MIN_PASSWORD_LENGTH as u32 && value <= MAX_PASSWORD_LENGTH as u32
            })
            .unwrap_or(false);

        let mut generate_button = button("Generate password");
        if can_generate {
            generate_button = generate_button.on_press(Message::GeneratePressed);
        }

        let controls = Row::new()
            .spacing(12)
            .push(generate_button)
            .push(copy_button);

        let password_display = if let Some(password) = &self.generated_password {
            text(password).size(24)
        } else {
            text("Generate a password to see it here").size(16)
        };

        let status_text = if let Some(message) = &self.status_message {
            text(message.clone()).size(14)
        } else {
            text("")
        };

        let content = Column::new()
            .spacing(20)
            .align_x(alignment::Horizontal::Center)
            .push(header)
            .push(length_controls)
            .push(toggles)
            .push(controls)
            .push(password_display)
            .push(status_text);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .padding(32)
            .into()
    }
}
