use crate::client::audit_db::{load_all_audit_events_for_ui, AuditEventDisplay};
use chrono::{TimeZone, Utc};
use iced::alignment;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Length, Task};

fn format_timestamp(timestamp_ms: i64) -> String {
    let seconds = timestamp_ms / 1000;
    let millis = (timestamp_ms % 1000) as u32;
    Utc.timestamp_opt(seconds, millis * 1_000_000)
        .single()
        .map(|dt| dt.format("%d.%m.%Y %H:%M:%S").to_string())
        .unwrap_or_else(|| timestamp_ms.to_string())
}

pub struct AuditPage {
    events: Vec<AuditEventDisplay>,
    loading: bool,
    status_message: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<Vec<AuditEventDisplay>, String>),
    RefreshPressed,
}

impl AuditPage {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                events: Vec::new(),
                loading: true,
                status_message: Some("Loading audit log...".into()),
            },
            Self::load_task(),
        )
    }

    fn load_task() -> Task<Message> {
        Task::perform(load_all_audit_events_for_ui(), Message::Loaded)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loaded(result) => {
                self.loading = false;
                match result {
                    Ok(events) => {
                        self.events = events;
                        self.status_message = Some(format!(
                            "Loaded {} audit events.",
                            self.events.len()
                        ));
                    }
                    Err(err) => {
                        self.events.clear();
                        self.status_message = Some(err);
                    }
                }
            }
            Message::RefreshPressed => {
                self.loading = true;
                self.status_message = Some("Refreshing audit log...".into());
                return Self::load_task();
            }
        }

        Task::none()
    }

    fn event_list(&self) -> Element<'_, Message> {
        if self.loading {
            return text("Loading...").size(16).into();
        }

        if self.events.is_empty() {
            return text("No audit events yet. Run m165_demo to seed data.")
                .size(14)
                .into();
        }

        let list = column(self.events.iter().map(|event| {
            let status = if event.success { "ok" } else { "failed" };
            text(format!(
                "[{}] {} — {} — {} ({status})",
                event.user_id,
                event.event_type,
                event.entry_title,
                format_timestamp(event.event_time_ms)
            ))
            .size(14)
            .width(Length::Fill)
            .into()
        }))
        .spacing(6)
        .width(Length::Fill);

        scrollable(list)
            .width(Length::Fill)
            .height(Length::FillPortion(3))
            .into()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let header = text("Audit Log")
            .size(32)
            .align_x(alignment::Horizontal::Center);

        let controls = row![button("Refresh").on_press(Message::RefreshPressed),]
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
            text(format!("All events ({})", self.events.len())).size(14),
            self.event_list(),
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
