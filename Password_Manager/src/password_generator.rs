use rand::{seq::SliceRandom, thread_rng, Rng};

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SPECIAL: &[u8] = b"!@#$%^&*()-_=+[]{};:,.<>/?";
pub const MAX_PASSWORD_LENGTH: usize = 128;

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
