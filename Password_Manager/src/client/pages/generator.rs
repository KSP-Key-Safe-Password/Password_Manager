use std::{fmt, sync::Arc};

use crate::client::{
    app::Message as AppMessage,
    framework::{
        Controller,
        ControllerCore,
        PageSurface,
    },
};
use crate::password_generator::{
    PasswordEngine,
    PasswordGenerationError,
    PasswordGenerator,
    PasswordSettings,
    MAX_PASSWORD_LENGTH,
};
use arboard::Clipboard;
use iced::alignment;
use iced::widget::{
    button, checkbox, container, text, text_input, Column, Row,
};
use iced::{Element, Length, Task};
use inherit::Inherit;

const MIN_PASSWORD_LENGTH: u16 = 4;

#[derive(Inherit)]
pub struct GeneratorPage {
    surface: PageSurface,
    engine: PasswordGenerator,
    generated_password: Option<String>,
    status_message: Option<String>,
    length_value: u16,
    length_input: String,
    clipboard: Box<dyn ClipboardGateway>,
}

impl GeneratorPage {
    pub fn new() -> (Self, Task<Message>) {
        let settings = PasswordSettings::default();
        let engine = PasswordGenerator::new(settings.clone());
        (
            Self {
                surface: PageSurface::new("Password Manager"),
                length_value: settings.length as u16,
                length_input: settings.length.to_string(),
                engine,
                generated_password: None,
                status_message: None,
                clipboard: Self::build_clipboard_gateway(),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        message.dispatch(self)
    }

    pub fn view(&self) -> Element<'_, Message> {
        let header = text(self.title())
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
                    .on_input(Message::length_input_changed),
            );

        let toggles = Column::new()
            .spacing(8)
            .width(Length::FillPortion(2))
            .push(
                checkbox(
                    "Include uppercase letters",
                    self.engine.settings().include_uppercase,
                )
                .on_toggle(Message::toggle_uppercase),
            )
            .push(
                checkbox("Include numbers", self.engine.settings().include_numbers)
                    .on_toggle(Message::toggle_numbers),
            )
            .push(
                checkbox(
                    "Include special characters",
                    self.engine.settings().include_special_chars,
                )
                .on_toggle(Message::toggle_special),
            );

        let mut copy_button = button("Copy to clipboard");
        if self.generated_password.is_some() {
            copy_button = copy_button.on_press(Message::copy_pressed());
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
            generate_button = generate_button.on_press(Message::generate_pressed());
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

    fn set_length_from_input(&mut self, value: &str) -> Task<Message> {
        let filtered: String = value.chars().filter(|ch| ch.is_ascii_digit()).collect();
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
                    self.engine.settings_mut().length = max as usize;
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
                        self.engine.settings_mut().length = parsed as usize;
                    }
                }
            }
            Err(_) => {
                self.length_input = filtered;
                status_message = Some("Please enter a valid number.".into());
            }
        }

        self.status_message = status_message;
        Task::none()
    }

    fn toggle_uppercase(&mut self, value: bool) -> Task<Message> {
        self.engine.settings_mut().include_uppercase = value;
        self.status_message = None;
        Task::none()
    }

    fn toggle_numbers(&mut self, value: bool) -> Task<Message> {
        self.engine.settings_mut().include_numbers = value;
        self.status_message = None;
        Task::none()
    }

    fn toggle_special(&mut self, value: bool) -> Task<Message> {
        self.engine.settings_mut().include_special_chars = value;
        self.status_message = None;
        Task::none()
    }

    fn generate_password(&mut self) -> Task<Message> {
        match self.engine.generate() {
            Ok(password) => {
                self.generated_password = Some(password);
                self.status_message = Some("Generated password.".into());
            }
            Err(err) => {
                self.generated_password = None;
                self.status_message = Some(Self::format_generation_error(err));
            }
        }

        Task::none()
    }

    fn copy_password(&mut self) -> Task<Message> {
        if let Some(password) = &self.generated_password {
            match self.clipboard.copy_text(password) {
                Ok(()) => {
                    self.status_message = Some("Password copied to clipboard.".into());
                }
                Err(err) => {
                    self.status_message = Some(format!("Failed to copy password: {err}"));
                }
            }
        } else {
            self.status_message = Some("No password to copy yet.".into());
        }

        Task::none()
    }

    fn build_clipboard_gateway() -> Box<dyn ClipboardGateway> {
        SystemClipboardGateway::new()
            .map(|gateway| Box::new(gateway) as Box<dyn ClipboardGateway>)
            .unwrap_or_else(|_| Box::new(NullClipboardGateway::default()))
    }

    fn format_generation_error(error: PasswordGenerationError) -> String {
        match error {
            PasswordGenerationError::EmptyLength => {
                "Password length must be greater than zero.".into()
            }
            PasswordGenerationError::ExceedsMaxLength => format!(
                "Password length exceeds supported maximum of {} characters.",
                MAX_PASSWORD_LENGTH
            ),
            PasswordGenerationError::TooShortForSelectedGroups => {
                "Password length too short for the selected character groups.".into()
            }
            PasswordGenerationError::InvalidUtf8 => {
                "Generated password contains invalid UTF-8 characters.".into()
            }
        }
    }
}

#[derive(Inherit)]
pub struct GeneratorController {
    core: ControllerCore,
    page: GeneratorPage,
}

impl GeneratorController {
    pub fn bootstrap() -> (Box<dyn Controller>, Task<AppMessage>) {
        let (page, task) = GeneratorPage::new();
        (
            Box::new(Self {
                core: ControllerCore::new("generator"),
                page,
            }),
            task.map(AppMessage::Generator),
        )
    }
}

impl Controller for GeneratorController {
    fn core(&self) -> &ControllerCore {
        self
    }

    fn accepts(&self, message: &AppMessage) -> bool {
        matches!(message, AppMessage::Generator(_))
    }

    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::Generator(inner) => self
                .page
                .update(inner)
                .map(AppMessage::Generator),
        }
    }

    fn view(&self) -> Element<'_, AppMessage> {
        self.page.view().map(AppMessage::Generator)
    }
}

#[derive(Clone)]
pub struct Message {
    action: Arc<dyn GeneratorAction>,
}

impl Message {
    pub fn length_input_changed(value: String) -> Self {
        Self::new(LengthInputChanged { value })
    }

    pub fn toggle_uppercase(value: bool) -> Self {
        Self::new(ToggleUppercase { value })
    }

    pub fn toggle_numbers(value: bool) -> Self {
        Self::new(ToggleNumbers { value })
    }

    pub fn toggle_special(value: bool) -> Self {
        Self::new(ToggleSpecial { value })
    }

    pub fn generate_pressed() -> Self {
        Self::new(GeneratePressed)
    }

    pub fn copy_pressed() -> Self {
        Self::new(CopyPressed)
    }

    fn new(action: impl GeneratorAction + 'static) -> Self {
        Self {
            action: Arc::new(action),
        }
    }

    fn dispatch(&self, page: &mut GeneratorPage) -> Task<Message> {
        self.action.apply(page)
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.action.fmt(f)
    }
}

trait GeneratorAction: fmt::Debug + Send + Sync {
    fn apply(&self, page: &mut GeneratorPage) -> Task<Message>;
}

#[derive(Debug)]
struct LengthInputChanged {
    value: String,
}

impl GeneratorAction for LengthInputChanged {
    fn apply(&self, page: &mut GeneratorPage) -> Task<Message> {
        page.set_length_from_input(&self.value)
    }
}

#[derive(Debug)]
struct ToggleUppercase {
    value: bool,
}

impl GeneratorAction for ToggleUppercase {
    fn apply(&self, page: &mut GeneratorPage) -> Task<Message> {
        page.toggle_uppercase(self.value)
    }
}

#[derive(Debug)]
struct ToggleNumbers {
    value: bool,
}

impl GeneratorAction for ToggleNumbers {
    fn apply(&self, page: &mut GeneratorPage) -> Task<Message> {
        page.toggle_numbers(self.value)
    }
}

#[derive(Debug)]
struct ToggleSpecial {
    value: bool,
}

impl GeneratorAction for ToggleSpecial {
    fn apply(&self, page: &mut GeneratorPage) -> Task<Message> {
        page.toggle_special(self.value)
    }
}

#[derive(Debug)]
struct GeneratePressed;

impl GeneratorAction for GeneratePressed {
    fn apply(&self, page: &mut GeneratorPage) -> Task<Message> {
        page.generate_password()
    }
}

#[derive(Debug)]
struct CopyPressed;

impl GeneratorAction for CopyPressed {
    fn apply(&self, page: &mut GeneratorPage) -> Task<Message> {
        page.copy_password()
    }
}

trait ClipboardGateway {
    fn copy_text(&mut self, value: &str) -> Result<(), String>;
}

struct SystemClipboardGateway {
    clipboard: Clipboard,
}

impl SystemClipboardGateway {
    fn new() -> Result<Self, String> {
        Clipboard::new()
            .map(|clipboard| Self { clipboard })
            .map_err(|err| err.to_string())
    }
}

impl ClipboardGateway for SystemClipboardGateway {
    fn copy_text(&mut self, value: &str) -> Result<(), String> {
        self.clipboard
            .set_text(value.to_owned())
            .map_err(|err| err.to_string())
    }
}

#[derive(Default)]
struct NullClipboardGateway;

impl ClipboardGateway for NullClipboardGateway {
    fn copy_text(&mut self, _value: &str) -> Result<(), String> {
        Err("Clipboard is not available on this platform.".into())
    }
}
