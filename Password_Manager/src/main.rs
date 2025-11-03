mod password_generator;

use arboard::Clipboard;
use password_generator::{ask_user_for_password_settings, generate_password};
use std::io::{self, Write};

fn main() {
    let settings = ask_user_for_password_settings();
    match generate_password(&settings) {
        Ok(password) => {
            println!("Generated password: {password}");
            if prompt_save_to_clipboard() {
                match copy_to_clipboard(&password) {
                    Ok(()) => println!("Password copied to clipboard."),
                    Err(err) => eprintln!("{err}"),
                }
            }
        }
        Err(err) => eprintln!("Could not create password: {err}"),
    }
}

fn prompt_save_to_clipboard() -> bool {
    loop {
        print!("Save password to clipboard? (y/n, default n): ");
        if io::stdout().flush().is_err() {
            return false;
        }

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return false;
        }

        match input.trim().to_lowercase().as_str() {
            "" => return false,
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please answer with 'y' or 'n'."),
        }
    }
}

fn copy_to_clipboard(password: &str) -> Result<(), String> {
    let mut clipboard =
        Clipboard::new().map_err(|err| format!("Failed to access clipboard: {err}"))?;
    clipboard
        .set_text(password.to_owned())
        .map_err(|err| format!("Failed to copy password: {err}"))
}
