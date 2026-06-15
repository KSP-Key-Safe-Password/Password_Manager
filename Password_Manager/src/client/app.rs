use crate::client::pages::audit::{AuditPage, Message as AuditMessage};
use crate::client::pages::generator::{
    GeneratorPage,
    Message as GeneratorMessage,
};
use crate::client::pages::vault::{VaultPage, Message as VaultMessage};
use iced::widget::{button, column, container, row, text};
use iced::{alignment, Element, Length, Task};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Generator,
    Vault,
    Audit,
}

pub struct PasswordManagerApp {
    current_page: Page,
    generator: GeneratorPage,
    vault: VaultPage,
    audit: AuditPage,
}

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Page),
    Generator(GeneratorMessage),
    Vault(VaultMessage),
    Audit(AuditMessage),
}

impl PasswordManagerApp {
    fn new() -> (Self, Task<Message>) {
        let (generator, generator_task) = GeneratorPage::new();
        let (vault, vault_task) = VaultPage::new();
        let (audit, audit_task) = AuditPage::new();

        (
            Self {
                current_page: Page::Generator,
                generator,
                vault,
                audit,
            },
            Task::batch(vec![
                generator_task.map(Message::Generator),
                vault_task.map(Message::Vault),
                audit_task.map(Message::Audit),
            ]),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(page) => {
                self.current_page = page;

                return match page {
                    Page::Vault => self
                        .vault
                        .update(VaultMessage::RefreshPressed)
                        .map(Message::Vault),
                    Page::Audit => self
                        .audit
                        .update(AuditMessage::RefreshPressed)
                        .map(Message::Audit),
                    Page::Generator => Task::none(),
                };
            }
            Message::Generator(msg) => self.generator.update(msg).map(Message::Generator),
            Message::Vault(msg) => self.vault.update(msg).map(Message::Vault),
            Message::Audit(msg) => self.audit.update(msg).map(Message::Audit),
        }
    }

    fn navigation(&self) -> Element<'_, Message> {
        let generator_button = nav_button("Generator", Page::Generator, self.current_page);
        let vault_button = nav_button("Vault", Page::Vault, self.current_page);
        let audit_button = nav_button("Audit", Page::Audit, self.current_page);

        row![generator_button, vault_button, audit_button]
            .spacing(8)
            .width(Length::Fill)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    fn view(&self) -> Element<'_, Message> {
        let page_content = match self.current_page {
            Page::Generator => self.generator.view().map(Message::Generator),
            Page::Vault => self.vault.view().map(Message::Vault),
            Page::Audit => self.audit.view().map(Message::Audit),
        };

        let content = column![self.navigation(), page_content]
            .spacing(16)
            .max_width(800);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .into()
    }
}

fn nav_button(label: &str, page: Page, current: Page) -> Element<'_, Message> {
    if current == page {
        button(text(label).width(Length::Fill)).into()
    } else {
        button(text(label).width(Length::Fill))
            .on_press(Message::Navigate(page))
            .into()
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
