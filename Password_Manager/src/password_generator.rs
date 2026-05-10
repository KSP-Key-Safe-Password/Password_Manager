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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn settings_with_length(length: usize) -> PasswordSettings {
        PasswordSettings {
            length,
            ..PasswordSettings::default()
        }
    }

    #[test]
    fn min_password_length_constant_is_four() {
        assert_eq!(MIN_PASSWORD_LENGTH, 4);
    }

    #[test]
    fn validate_rejects_length_below_minimum() {
        let settings = settings_with_length(MIN_PASSWORD_LENGTH - 1);
        let result = validate_password_settings(&settings);
        assert!(result.is_err(), "expected validation error for length 3");
    }

    #[test]
    fn validate_accepts_minimum_length() {
        let settings = settings_with_length(MIN_PASSWORD_LENGTH);
        assert!(validate_password_settings(&settings).is_ok());
    }

    #[test]
    fn validate_rejects_length_above_maximum() {
        let settings = settings_with_length(MAX_PASSWORD_LENGTH + 1);
        assert!(validate_password_settings(&settings).is_err());
    }

    #[test]
    fn validate_accepts_maximum_length() {
        let settings = settings_with_length(MAX_PASSWORD_LENGTH);
        assert!(validate_password_settings(&settings).is_ok());
    }

    #[test]
    fn validate_rejects_when_length_below_active_pool_count() {
        let settings = PasswordSettings {
            length: 2,
            include_uppercase: true,
            include_numbers: true,
            include_special_chars: false,
        };
        assert!(validate_password_settings(&settings).is_err());
    }

    #[test]
    fn validate_default_settings_pass() {
        assert!(validate_password_settings(&PasswordSettings::default()).is_ok());
    }

    #[test]
    fn generate_with_rng_is_deterministic_for_same_seed() {
        let settings = PasswordSettings::default();
        let mut rng_a = StdRng::seed_from_u64(42);
        let mut rng_b = StdRng::seed_from_u64(42);

        let pwd_a = generate_password_with_rng(&settings, &mut rng_a).unwrap();
        let pwd_b = generate_password_with_rng(&settings, &mut rng_b).unwrap();

        assert_eq!(pwd_a, pwd_b);
    }

    #[test]
    fn generate_with_rng_respects_only_letters_pools() {
        let settings = PasswordSettings {
            length: 16,
            include_uppercase: true,
            include_numbers: false,
            include_special_chars: false,
        };
        let mut rng = StdRng::seed_from_u64(7);
        let pwd = generate_password_with_rng(&settings, &mut rng).unwrap();
        assert_eq!(pwd.len(), 16);
        assert!(
            pwd.chars().all(|c| c.is_ascii_alphabetic()),
            "expected only letters, got {pwd}"
        );
    }

    #[test]
    fn generate_with_rng_uses_all_pools_when_enabled() {
        let settings = PasswordSettings {
            length: 64,
            include_uppercase: true,
            include_numbers: true,
            include_special_chars: true,
        };
        let mut rng = StdRng::seed_from_u64(123);
        let pwd = generate_password_with_rng(&settings, &mut rng).unwrap();
        assert_eq!(pwd.len(), 64);
        assert!(pwd.chars().any(|c| c.is_ascii_lowercase()));
        assert!(pwd.chars().any(|c| c.is_ascii_uppercase()));
        assert!(pwd.chars().any(|c| c.is_ascii_digit()));
        assert!(pwd.chars().any(|c| !c.is_ascii_alphanumeric()));
    }

    #[test]
    fn generate_password_returns_correct_length() {
        let settings = settings_with_length(20);
        let pwd = generate_password(&settings).unwrap();
        assert_eq!(pwd.len(), 20);
    }
}
