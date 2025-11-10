use inherit::Inherit;
use rand::{
    rngs::ThreadRng,
    seq::SliceRandom,
    thread_rng,
    Rng,
};
use std::fmt;

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SPECIAL: &[u8] = b"!@#$%^&*()-_=+[]{};:,.<>/?";
pub const MAX_PASSWORD_LENGTH: usize = 128;

pub trait PasswordEngine {
    fn settings(&self) -> &PasswordSettings;
    fn settings_mut(&mut self) -> &mut PasswordSettings;
    fn generate(&self) -> Result<String, PasswordGenerationError>;
}

pub trait PasswordAlgorithm: Send + Sync {
    fn generate(
        &self,
        runtime: &mut GenerationRuntime<'_>,
    ) -> Result<String, PasswordGenerationError>;
}

pub trait CharacterPalette: Send + Sync {
    fn compose(&self, settings: &PasswordSettings) -> Vec<&'static [u8]>;
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct PasswordEngineBase {
    settings: PasswordSettings,
}

impl PasswordEngineBase {
    pub fn new(settings: PasswordSettings) -> Self {
        Self { settings }
    }

    pub fn settings(&self) -> &PasswordSettings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut PasswordSettings {
        &mut self.settings
    }
}

#[derive(Inherit)]
pub struct PasswordGenerator {
    base: PasswordEngineBase,
    algorithm: Box<dyn PasswordAlgorithm>,
}

impl PasswordGenerator {
    pub fn new(settings: PasswordSettings) -> Self {
        Self::with_algorithm(settings, Box::new(DefaultPasswordAlgorithm::default()))
    }

    pub fn with_algorithm(
        settings: PasswordSettings,
        algorithm: Box<dyn PasswordAlgorithm>,
    ) -> Self {
        Self {
            base: PasswordEngineBase::new(settings),
            algorithm,
        }
    }
}

impl PasswordEngine for PasswordGenerator {
    fn settings(&self) -> &PasswordSettings {
        self.base.settings()
    }

    fn settings_mut(&mut self) -> &mut PasswordSettings {
        self.base.settings_mut()
    }

    fn generate(&self) -> Result<String, PasswordGenerationError> {
        let mut runtime = GenerationRuntime::new(self.base.settings());
        self.algorithm.generate(&mut runtime)
    }
}

#[derive(Debug, Clone)]
pub enum PasswordGenerationError {
    EmptyLength,
    ExceedsMaxLength,
    TooShortForSelectedGroups,
    InvalidUtf8,
}

impl fmt::Display for PasswordGenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordGenerationError::EmptyLength => {
                write!(f, "Password length must be greater than zero")
            }
            PasswordGenerationError::ExceedsMaxLength => {
                write!(f, "Password length exceeds supported maximum")
            }
            PasswordGenerationError::TooShortForSelectedGroups => write!(
                f,
                "Password length too short for the selected character groups"
            ),
            PasswordGenerationError::InvalidUtf8 => {
                write!(f, "Generated password contains invalid UTF-8")
            }
        }
    }
}

impl std::error::Error for PasswordGenerationError {}

pub struct DefaultPasswordAlgorithm {
    palette: Box<dyn CharacterPalette>,
}

impl Default for DefaultPasswordAlgorithm {
    fn default() -> Self {
        Self {
            palette: Box::new(StandardCharacterPalette),
        }
    }
}

impl PasswordAlgorithm for DefaultPasswordAlgorithm {
    fn generate(
        &self,
        runtime: &mut GenerationRuntime<'_>,
    ) -> Result<String, PasswordGenerationError> {
        let pools = self.palette.compose(runtime.settings());
        runtime.validate_length(pools.len())?;
        runtime.build_password(&pools)
    }
}

struct StandardCharacterPalette;

impl CharacterPalette for StandardCharacterPalette {
    fn compose(&self, settings: &PasswordSettings) -> Vec<&'static [u8]> {
        let mut pools: Vec<&'static [u8]> = vec![LOWERCASE];

        if settings.include_uppercase {
            pools.push(UPPERCASE);
        }
        if settings.include_numbers {
            pools.push(NUMBERS);
        }
        if settings.include_special_chars {
            pools.push(SPECIAL);
        }

        pools
    }
}

pub struct GenerationRuntime<'a> {
    settings: &'a PasswordSettings,
    rng: ThreadRng,
}

impl<'a> GenerationRuntime<'a> {
    fn new(settings: &'a PasswordSettings) -> Self {
        Self {
            settings,
            rng: thread_rng(),
        }
    }

    pub fn settings(&self) -> &PasswordSettings {
        self.settings
    }

    fn validate_length(&self, pool_count: usize) -> Result<(), PasswordGenerationError> {
        if self.settings.length == 0 {
            return Err(PasswordGenerationError::EmptyLength);
        }
        if self.settings.length > MAX_PASSWORD_LENGTH {
            return Err(PasswordGenerationError::ExceedsMaxLength);
        }
        if self.settings.length < pool_count {
            return Err(PasswordGenerationError::TooShortForSelectedGroups);
        }

        Ok(())
    }

    fn build_password(
        &mut self,
        pools: &[&'static [u8]],
    ) -> Result<String, PasswordGenerationError> {
        if pools.is_empty() {
            return Err(PasswordGenerationError::TooShortForSelectedGroups);
        }

        let mut password = Vec::with_capacity(self.settings.length);

        for pool in pools {
            password.push(self.draw_from(pool));
        }

        while password.len() < self.settings.length {
            let pool = pools[self.rng.gen_range(0..pools.len())];
            password.push(self.draw_from(pool));
        }

        password.shuffle(&mut self.rng);
        String::from_utf8(password).map_err(|_| PasswordGenerationError::InvalidUtf8)
    }

    fn draw_from(&mut self, pool: &[u8]) -> u8 {
        pool[self.rng.gen_range(0..pool.len())]
    }
}
