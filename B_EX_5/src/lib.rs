#![allow(dead_code)]

pub mod config {
    pub mod model {
        use std::fmt;
        use std::str::FromStr;

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum FeatureState {
            Enabled,
            Disabled,
            Percentage(u8),
        }

        impl FeatureState {
            pub fn label(&self) -> String {
                match self {
                    FeatureState::Enabled => "enabled".to_string(),
                    FeatureState::Disabled => "disabled".to_string(),
                    FeatureState::Percentage(p) => format!("percentage:{}", p),
                }
            }
        }

        impl FromStr for FeatureState {
            type Err = ConfigError;

            fn from_str(raw: &str) -> Result<Self, Self::Err> {
                let trimmed = raw.trim().to_lowercase();
                match trimmed.as_str() {
                    "enabled" => Ok(FeatureState::Enabled),
                    "disabled" => Ok(FeatureState::Disabled),
                    s if s.starts_with("percentage:") => {
                        let num_str = &s[11..];
                        match num_str.parse::<u8>() {
                            Ok(num) if num <= 100 => Ok(FeatureState::Percentage(num)),
                            _ => Err(ConfigError::InvalidPercentage { raw: num_str.to_string() }),
                        }
                    }
                    _ => Err(ConfigError::InvalidState { raw: raw.to_string() }),
                }
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct FeatureFlag {
            pub scope: String,
            pub name: String,
            pub state: FeatureState,
        }

        impl FeatureFlag {
            pub fn from_line(line: &str) -> Result<Self, ConfigError> {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() != 2 {
                    return Err(ConfigError::InvalidFormat { line: line.to_string() });
                }

                let left = parts[0].trim();
                let right = parts[1].trim();
                let scope_name: Vec<&str> = left.split("::").collect();
                if scope_name.len() != 2 {
                    return Err(ConfigError::InvalidFormat { line: line.to_string() });
                }

                let scope = scope_name[0].trim().to_string();
                let name = scope_name[1].trim().to_string();
                let state = FeatureState::from_str(right)?;
                
                Ok(FeatureFlag { scope, name, state })
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum ConfigError {
            InvalidFormat { line: String },
            InvalidState { raw: String },
            InvalidPercentage { raw: String },
            DuplicateFlag { scope: String, name: String },
            Empty,
        }

        impl fmt::Display for ConfigError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    ConfigError::InvalidFormat { line } => write!(f, "Niepoprawny format linii: {}", line),
                    ConfigError::InvalidState { raw } => write!(f, "Nieznany stan flagi: {}", raw),
                    ConfigError::InvalidPercentage { raw } => write!(f, "Niepoprawny procent: {}", raw),
                    ConfigError::DuplicateFlag { scope, name } => write!(f, "Zduplikowana flaga: {}::{}", scope, name),
                    ConfigError::Empty => write!(f, "Brak flag konfiguracyjnych"),
                }
            }
        }

        impl std::error::Error for ConfigError {}
    }

    pub mod parser {
        use super::model::{ConfigError, FeatureFlag};
        use crate::FeatureRegistry;
        use std::io::BufRead;

        pub fn parse_flags<R: BufRead>(reader: R) -> Result<Vec<FeatureFlag>, ConfigError> {
            let mut flags = Vec::new();

            for line in reader.lines() {
                let line = line.map_err(|_| ConfigError::Empty)?; // Handle I/O errors
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue; // Skip empty lines and comments
                }

                flags.push(FeatureFlag::from_line(trimmed)?);
            }

            if flags.is_empty() {
                return Err(ConfigError::Empty);
            }
            Ok(flags)
        }

        pub fn load_registry<R: BufRead>(reader: R) -> Result<FeatureRegistry, ConfigError> {
            let flags = parse_flags(reader)?;
            FeatureRegistry::from_flags(flags)
        }
    }

    pub mod registry {
        use super::model::{ConfigError, FeatureFlag};
        use std::collections::BTreeMap;

        #[derive(Debug, Default)]
        pub struct FeatureRegistry {
            entries: BTreeMap<String, Vec<FeatureFlag>>,
        }

        impl FeatureRegistry {
            pub fn from_flags(flags: Vec<FeatureFlag>) -> Result<Self, ConfigError> {
                let mut registry = FeatureRegistry::default();
                for flag in flags {
                    registry.insert(flag)?;
                }
                Ok(registry)
            }

            pub fn insert(&mut self, flag: FeatureFlag) -> Result<(), ConfigError> {
                let scope = flag.scope.clone();
                let name = flag.name.clone();

                let entry = self.entries.entry(scope.clone()).or_insert_with(Vec::new);
                if entry.iter().any(|f| f.name == name) {
                    return Err(ConfigError::DuplicateFlag { scope, name });
                }

                entry.push(flag);
                entry.sort_by(|a, b| a.name.cmp(&b.name));
                Ok(())
            }

            pub fn flags_for(&self, scope: &str) -> Option<&[FeatureFlag]> {
                self.entries.get(scope).map(|v| v.as_slice())
            }

            pub fn scopes(&self) -> impl Iterator<Item = (&str, &[FeatureFlag])> {
                self.entries.iter().map(|(scope, flags)| (scope.as_str(), flags.as_slice()))
            }
        }

        #[cfg(feature = "preview")]
        pub fn render_preview(registry: &FeatureRegistry) -> String {
            let mut result = String::new();
            for (scope, flags) in registry.scopes() {
                for flag in flags {
                    result.push_str(&format!("{}::{} -> {}\n", scope, flag.name, flag.state.label()));
                }
            }
            result
        }
    }
}

pub use config::model::{ConfigError, FeatureFlag, FeatureState};
pub use config::parser::{load_registry, parse_flags};
pub use config::registry::FeatureRegistry;

pub mod prelude {
    pub use super::{load_registry, parse_flags, FeatureFlag, FeatureRegistry, FeatureState};
}

#[cfg(feature = "preview")]
pub use config::registry::render_preview;
