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
                    FeatureState::Percentage(percentage) => format!("percentage:{}", percentage)
                }
            }
        }

        impl FromStr for FeatureState {
            type Err = ConfigError;

            fn from_str(raw: &str) -> Result<Self, Self::Err> {
                match raw.to_lowercase().as_str() {
                    "enabled" => Ok(FeatureState::Enabled),
                    "disabled" => Ok(FeatureState::Disabled),
                    percentage_candidate => {
                        let (prefix, val) = percentage_candidate
                            .split_once(":")
                            .filter(|(prefix, percentage)| prefix.to_lowercase() == "percentage")
                            .ok_or_else(|| ConfigError::InvalidState { raw: raw.to_string()})?;

                        let percentage = val.parse::<u8>()
                            .map_err(|f| ConfigError::InvalidPercentage { raw: raw.to_string() })?;

                        if percentage <= 100 {
                            Ok(FeatureState::Percentage(percentage))
                        } else {
                            Err(ConfigError::InvalidPercentage { raw: raw.to_string() })
                        }
                    }
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
                let (scope_and_name, state) = line
                    .split_once("=")
                    .ok_or_else(|| ConfigError::InvalidFormat { line: line.to_string()})?;

                let state = FeatureState::from_str(state.trim())?;
                let (scope, name) = scope_and_name
                    .split_once("::")
                    .map(|(scope, name)| (scope.trim().to_string(), name.trim().to_string()))
                    .ok_or_else(|| ConfigError::InvalidFormat { line: line.to_string()})?;

                if scope.is_empty() || name.is_empty() {
                    return Err(ConfigError::InvalidFormat { line: line.to_string() });
                }

                Ok(FeatureFlag { scope, name, state})
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
                    ConfigError::Empty => write!(f, "Brak flag konfiguracyjnych")
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
            let mut results: Vec<FeatureFlag> = vec!();
            for line in reader.lines() {
                let line = line.unwrap();
                if line.trim().is_empty() || line.trim().starts_with("#") {
                    continue;
                }
                results.push(FeatureFlag::from_line(line.as_str())?)
            }
            if results.is_empty() {
                Err(ConfigError::Empty)
            } else {
                Ok(results)
            }
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
                let mut registry = FeatureRegistry { entries: BTreeMap::new() };
                for flag in flags {
                    registry.insert(flag)?;
                }
                Ok(registry)
            }

            pub fn insert(&mut self, flag: FeatureFlag) -> Result<(), ConfigError> {
                let flags = self.entries.entry(flag.scope.clone())
                    .or_insert_with(|| vec![]);
                if flags.iter().any(|f| f.name == flag.name) {
                    Err(ConfigError::DuplicateFlag {scope: flag.scope.clone(), name: flag.name.clone()})
                } else {
                    flags.push(flag);
                    flags.sort_by(|a, b| a.name.cmp(&b.name));
                    Ok(())
                }
            }

            pub fn flags_for(&self, scope: &str) -> Option<&[FeatureFlag]> {
                self.entries.get(scope).map(|flags| flags.as_slice())
            }

            pub fn scopes(&self) -> impl Iterator<Item = (&str, &[FeatureFlag])> {
                self.entries.iter().map(|(e,f)| (e.as_str(), f.as_slice()))
            }
        }

        #[cfg(feature = "preview")]
        pub fn render_preview(registry: &FeatureRegistry) -> String {
            let mut lines = vec![];
            for (scope, flags) in registry.scopes() {
                for flag in flags {
                    lines.push(format!("{}::{} -> {}\n", scope, flag.name, flag.state.label()))
                }
            }
            lines.concat()
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
