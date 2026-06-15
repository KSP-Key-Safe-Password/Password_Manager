pub trait ClipboardWriter {
    fn set_text(&mut self, text: &str) -> Result<(), String>;
}

pub struct RealClipboard {
    inner: arboard::Clipboard,
}

impl RealClipboard {
    pub fn new() -> Result<Self, String> {
        arboard::Clipboard::new()
            .map(|inner| Self { inner })
            .map_err(|e| e.to_string())
    }
}

impl ClipboardWriter for RealClipboard {
    fn set_text(&mut self, text: &str) -> Result<(), String> {
        self.inner
            .set_text(text.to_string())
            .map_err(|e| e.to_string())
    }
}

pub fn copy_password_to_clipboard(
    clipboard: &mut dyn ClipboardWriter,
    password: Option<&str>,
) -> String {
    match password {
        Some(pw) => match clipboard.set_text(pw) {
            Ok(()) => "Password copied to clipboard.".into(),
            Err(e) => format!("Failed to copy password: {e}"),
        },
        None => "No password to copy yet.".into(),
    }
}

#[cfg(test)]
pub struct SpyClipboard {
    pub last_written: Option<String>,
}

#[cfg(test)]
impl SpyClipboard {
    pub fn new() -> Self {
        Self { last_written: None }
    }
}

#[cfg(test)]
impl ClipboardWriter for SpyClipboard {
    fn set_text(&mut self, text: &str) -> Result<(), String> {
        self.last_written = Some(text.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spy_records_written_text() {
        let mut spy = SpyClipboard::new();
        spy.set_text("hunter2").unwrap();
        assert_eq!(spy.last_written, Some("hunter2".to_string()));
    }

    #[test]
    fn copy_helper_writes_correct_password() {
        let mut spy = SpyClipboard::new();
        copy_password_to_clipboard(&mut spy, Some("abc123"));
        assert_eq!(spy.last_written, Some("abc123".to_string()));
    }

    #[test]
    fn copy_helper_does_nothing_without_password() {
        let mut spy = SpyClipboard::new();
        copy_password_to_clipboard(&mut spy, None);
        assert_eq!(spy.last_written, None);
    }
}
