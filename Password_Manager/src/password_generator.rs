use rand::{seq::SliceRandom, thread_rng, Rng};
use std::io::{self, Write};

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SPECIAL: &[u8] = b"!@#$%^&*()-_=+[]{};:,.<>/?";
const MAX_PASSWORD_LENGTH: usize = 128;

pub struct PasswordSettings {
    pub length: usize,
    pub include_uppercase: bool,
    pub include_numbers: bool,
    pub include_special_chars: bool,
}
impl Default for PasswordSettings {
    fn default() -> Self {
        PasswordSettings {
            length: 12,
            include_uppercase: true,
            include_numbers: true,
            include_special_chars: true,
        }
    }
}

pub fn generate_password(settings: &PasswordSettings) -> Result<String, &'static str> {
    if settings.length == 0 {
        return Err("Password length must be greater than zero");
    }
    if settings.length > MAX_PASSWORD_LENGTH {
        return Err("Password length exceeds supported maximum");
    }

    let mut pools: Vec<&[u8]> = vec![LOWERCASE];
    if settings.include_uppercase {
        pools.push(UPPERCASE);
    }
    if settings.include_numbers {
        pools.push(NUMBERS);
    }
    if settings.include_special_chars {
        pools.push(SPECIAL);
    }

    if settings.length < pools.len() {
        return Err("Password length too short for the selected character groups");
    }

    let mut rng = thread_rng();
    let mut password = Vec::with_capacity(settings.length);

    for pool in &pools {
        password.push(pool[rng.gen_range(0..pool.len())]);
    }

    while password.len() < settings.length {
        let pool = pools[rng.gen_range(0..pools.len())];
        password.push(pool[rng.gen_range(0..pool.len())]);
    }

    password.shuffle(&mut rng);

    String::from_utf8(password).map_err(|_| "Generated password contains invalid UTF-8")
}

pub fn ask_user_for_password_settings() -> PasswordSettings {
    println!("Configure your password settings:");
    let defaults = PasswordSettings::default();
    let length = prompt_usize("Enter desired password length", defaults.length);
    let include_uppercase =
        prompt_bool("Include uppercase letters?", defaults.include_uppercase);
    let include_numbers = prompt_bool("Include numbers?", defaults.include_numbers);
    let include_special_chars =
        prompt_bool("Include special characters?", defaults.include_special_chars);

    set_password_settings(
        length,
        include_uppercase,
        include_numbers,
        include_special_chars,
    )
}

fn set_password_settings(
    length: usize,
    include_uppercase: bool,
    include_numbers: bool,
    include_special_chars: bool,
) -> PasswordSettings {
    PasswordSettings {
        length,
        include_uppercase,
        include_numbers,
        include_special_chars,
    }
}

fn prompt_input(prompt: &str) -> Option<String> {
    print!("{prompt}: ");
    if io::stdout().flush().is_err() {
        return None;
    }

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return None;
    }
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn prompt_usize(prompt: &str, default: usize) -> usize {
    loop {
        let full_prompt = format!(
            "{prompt} (default {default}, max {MAX_PASSWORD_LENGTH})"
        );
        match prompt_input(&full_prompt) {
            Some(value) => match value.parse() {
                Ok(parsed) if parsed <= MAX_PASSWORD_LENGTH => return parsed,
                Ok(_) => {
                    println!("Please enter a number up to {MAX_PASSWORD_LENGTH}.");
                    continue;
                }
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue;
                }
            },
            None => return default,
        }
    }
}

fn prompt_bool(prompt: &str, default: bool) -> bool {
    let default_hint = if default { "y" } else { "n" };
    loop {
        let full_prompt = format!("{prompt} (y/n, default {default_hint})");
        match prompt_input(&full_prompt) {
            Some(value) => match value.to_lowercase().as_str() {
                "y" | "yes" => return true,
                "n" | "no" => return false,
                _ => {
                    println!("Please answer with 'y' or 'n'.");
                    continue;
                }
            },
            None => return default,
        }
    }
}
