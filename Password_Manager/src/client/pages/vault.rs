use crate::clipboard::{RealClipboard, copy_password_to_clipboard};
use crate::client::vault_db::{
    display_password, load_vault_entries_for_ui, save_vault_entry_for_ui,
    vault_entry_from_password,
};
use Password_Manager::db::models::VaultEntry;
use chrono::{TimeZone, Utc};
use iced::alignment;
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Element, Length, Task};

#[derive(Debug, Clone, Default)]
struct EntryForm {
    password: String,
}

impl EntryForm {
    fn clear(&mut self) {
        self.password.clear();
    }

    fn can_save(&self) -> bool {
        !self.password.trim().is_empty()
    }

    fn to_entry(&self) -> Result<VaultEntry, String> {
        vault_entry_from_password(&self.password)
    }
}

fn format_timestamp(timestamp: i64) -> String {
    Utc.timestamp_opt(timestamp, 0)
        .single()
        .map(|dt| dt.format("%d.%m.%Y %H:%M").to_string())
        .unwrap_or_else(|| timestamp.to_string())
}

pub struct VaultPage {
    entries: Vec<VaultEntry>,
    filtered_indices: Vec<usize>,
    loading: bool,
    status_message: Option<String>,
    search_query: String,
    selected_index: Option<usize>,
    add_form: EntryForm,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<Vec<VaultEntry>, String>),
    RefreshPressed,
    SearchChanged(String),
    EntrySelected(usize),
    CopyPasswordPressed,
    AddPasswordChanged(String),
    AddEntryPressed,
    EntrySaved(Result<(), String>),
}

impl VaultPage {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                entries: Vec::new(),
                filtered_indices: Vec::new(),
                loading: true,
                status_message: Some("Loading vault entries...".into()),
                search_query: String::new(),
                selected_index: None,
                add_form: EntryForm::default(),
            },
            Self::load_task(),
        )
    }

    fn load_task() -> Task<Message> {
        Task::perform(load_vault_entries_for_ui(), Message::Loaded)
    }

    fn apply_filter(&mut self) {
        let query = self.search_query.to_lowercase();
        self.filtered_indices = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| {
                if query.is_empty() {
                    return true;
                }
                display_password(entry).to_lowercase().contains(&query)
            })
            .map(|(index, _)| index)
            .collect();

        if let Some(selected) = self.selected_index {
            if !self.filtered_indices.contains(&selected) {
                self.selected_index = self.filtered_indices.first().copied();
            }
        } else {
            self.selected_index = self.filtered_indices.first().copied();
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loaded(result) => {
                self.loading = false;
                match result {
                    Ok(entries) => {
                        self.entries = entries;
                        self.apply_filter();
                        self.status_message = Some(format!(
                            "Loaded {} passwords from MongoDB.",
                            self.entries.len()
                        ));
                    }
                    Err(err) => {
                        self.entries.clear();
                        self.filtered_indices.clear();
                        self.selected_index = None;
                        self.status_message = Some(err);
                    }
                }
            }
            Message::RefreshPressed => {
                self.loading = true;
                self.status_message = Some("Refreshing vault entries...".into());
                return Self::load_task();
            }
            Message::SearchChanged(value) => {
                self.search_query = value;
                self.apply_filter();
            }
            Message::EntrySelected(index) => {
                self.selected_index = Some(index);
            }
            Message::CopyPasswordPressed => {
                if let Some(index) = self.selected_index {
                    if let Some(entry) = self.entries.get(index) {
                        match RealClipboard::new() {
                            Ok(mut clipboard) => {
                                self.status_message = Some(copy_password_to_clipboard(
                                    &mut clipboard,
                                    Some(&display_password(entry)),
                                ));
                            }
                            Err(e) => {
                                self.status_message =
                                    Some(format!("Could not open clipboard: {e}"));
                            }
                        }
                    }
                }
            }
            Message::AddPasswordChanged(value) => self.add_form.password = value,
            Message::AddEntryPressed => match self.add_form.to_entry() {
                Ok(entry) => {
                    self.status_message = Some("Saving password to vault...".into());
                    return Task::perform(save_vault_entry_for_ui(entry), Message::EntrySaved);
                }
                Err(err) => {
                    self.status_message = Some(err);
                }
            },
            Message::EntrySaved(result) => match result {
                Ok(()) => {
                    self.add_form.clear();
                    self.status_message = Some("Password saved to vault.".into());
                    return Self::load_task();
                }
                Err(err) => {
                    self.status_message = Some(err);
                }
            },
        }

        Task::none()
    }

    fn add_entry_form(&self) -> Element<'_, Message> {
        let mut save_button = button("Save to vault");
        if self.add_form.can_save() {
            save_button = save_button.on_press(Message::AddEntryPressed);
        }

        let form = column![
            text("Add password manually").size(20),
            text_input("Password", &self.add_form.password).on_input(Message::AddPasswordChanged),
            save_button,
        ]
        .spacing(8)
        .width(Length::Fill);

        container(form).padding(12).width(Length::Fill).into()
    }

    fn entry_list(&self) -> Element<'_, Message> {
        if self.loading {
            return text("Loading...").size(16).into();
        }

        if self.entries.is_empty() {
            return text("No passwords yet. Add one manually or save from the generator.")
                .size(14)
                .into();
        }

        if self.filtered_indices.is_empty() {
            return text("No passwords match your search.").size(14).into();
        }

        let list = column(self.filtered_indices.iter().map(|&index| {
            let entry = &self.entries[index];
            let label = format!(
                "{}  ({})",
                display_password(entry),
                format_timestamp(entry.created_at)
            );
            let is_selected = self.selected_index == Some(index);

            let entry_button = if is_selected {
                button(text(label).width(Length::Fill))
            } else {
                button(text(label).width(Length::Fill)).on_press(Message::EntrySelected(index))
            };

            entry_button.width(Length::Fill).padding(8).into()
        }))
        .spacing(4)
        .width(Length::Fill);

        scrollable(list)
            .width(Length::Fill)
            .height(Length::FillPortion(2))
            .into()
    }

    fn entry_details(&self) -> Element<'_, Message> {
        let Some(index) = self.selected_index else {
            return text("Select a password to view details.")
                .size(14)
                .width(Length::Fill)
                .into();
        };

        let Some(entry) = self.entries.get(index) else {
            return text("Selected password is no longer available.")
                .size(14)
                .width(Length::Fill)
                .into();
        };

        let details = column![
            text("Password").size(20),
            text(display_password(entry)).size(24),
            text(format!("Saved: {}", format_timestamp(entry.created_at))),
            button("Copy password").on_press(Message::CopyPasswordPressed),
        ]
        .spacing(8)
        .width(Length::Fill);

        container(details)
            .padding(12)
            .width(Length::Fill)
            .into()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let header = text("Vault")
            .size(32)
            .align_x(alignment::Horizontal::Center);

        let controls = row![
            text_input("Search passwords...", &self.search_query).on_input(Message::SearchChanged),
            button("Refresh").on_press(Message::RefreshPressed),
        ]
        .spacing(12)
        .align_y(alignment::Vertical::Center);

        let status = text(
            self.status_message
                .clone()
                .unwrap_or_else(|| "Ready.".into()),
        )
        .size(14);

        let content = column![
            header,
            controls,
            self.add_entry_form(),
            text(format!(
                "Showing {} of {} passwords",
                self.filtered_indices.len(),
                self.entries.len()
            ))
            .size(14),
            self.entry_list(),
            self.entry_details(),
            status,
        ]
        .spacing(16)
        .width(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(32)
            .into()
    }
}
