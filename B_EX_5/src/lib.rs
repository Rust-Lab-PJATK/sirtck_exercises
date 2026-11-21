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
                    FeatureState::Percentage(p) => format!("percentage:{}", p)
                }
            }
        }

        impl FromStr for FeatureState {
            type Err = ConfigError;

            fn from_str(raw: &str) -> Result<Self, Self::Err> {
                let lower = raw.to_string().to_ascii_lowercase();
                match lower.as_str() {
                    "enabled" => return Ok(FeatureState::Enabled),
                    "disabled" => return Ok(FeatureState::Disabled),
                    _ => {}
                };
                let (key, value) = raw.split_once(':').ok_or_else(|| ConfigError::InvalidState { raw: raw.to_string() })?;
                let lower = key.to_string().to_ascii_lowercase();
                if lower != "percentage" {
                    return Err(ConfigError::InvalidState { raw: raw.to_string() });
                }
                let percentage = value.parse::<u8>().map_err(|_| ConfigError::InvalidState { raw: raw.to_string() })?;
                if percentage > 100 {
                    return Err(ConfigError::InvalidPercentage { raw: raw.to_string() });
                }
                Ok(FeatureState::Percentage(percentage))
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
                if line.is_empty() {
                    return Err(ConfigError::Empty);
                }
                let (scope, remainder) = line.split_once("::")
                    .ok_or_else(|| ConfigError::InvalidFormat { line: line.to_string() })?;
                let (name, state) = remainder.split_once("=")
                    .ok_or_else(|| ConfigError::InvalidFormat { line: line.to_string() })?;
                let scope = scope.trim().to_string();
                if scope.is_empty() {
                    return Err(ConfigError::InvalidFormat { line: line.to_string() });
                }
                let name = name.trim().to_string();
                if name.is_empty() {
                    return Err(ConfigError::InvalidFormat { line: line.to_string() });
                }
                let state = state.trim();
                if state.is_empty() {
                    return Err(ConfigError::InvalidFormat { line: line.to_string() });
                }
                let state = FeatureState::from_str(state)?;
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
            let mut flags = Vec::<FeatureFlag>::new();
            for line in reader.lines() {
                let line = line.unwrap();
                let line = line.trim();
                if line.is_empty() || line.starts_with("#") {
                    continue;
                }
                let flag = FeatureFlag::from_line(line)?;
                flags.push(flag);
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
                let mut map = BTreeMap::<String, Vec<FeatureFlag>>::new();
                for flag in flags {
                    let flags = map.get_mut(&flag.scope);
                    if flags == None {
                        let scope = flag.scope.clone();
                        let flags = vec![flag];
                        map.insert(scope, flags);
                        continue;
                    }
                    let flags = flags.unwrap();
                    let mut index = 0;
                    loop {
                        if index == flags.len() {
                            flags.insert(index, flag);
                            break;
                        }
                        let other = &mut flags[index];
                        if other.name > flag.name {
                            flags.insert(index, flag);
                            break;
                        }
                        if other.name == flag.name {
                            let scope = flag.scope;
                            let name = flag.name;
                            return Err(ConfigError::DuplicateFlag { scope, name } );
                        }
                        index += 1;
                    }
                }
                Ok(FeatureRegistry { entries: map })
            }

            pub fn insert(&mut self, flag: FeatureFlag) -> Result<(), ConfigError> {
                let scope = &flag.scope;
                let flags = self.entries.get_mut(scope);
                if flags == None {
                    self.entries.insert(scope.clone(), vec![flag]);
                    return Ok(());
                }
                let flags = flags.unwrap();
                let mut index = 0;
                loop {
                    if index == flags.len() {
                        flags.insert(index, flag);
                        break;
                    }
                    let other = &mut flags[index];
                    if other.name > flag.name {
                        flags.insert(index, flag);
                        break;
                    }
                    if other.name == flag.name {
                        let scope = flag.scope;
                        let name = flag.name;
                        return Err(ConfigError::DuplicateFlag { scope, name } );
                    }
                    index += 1;
                }
                Ok(())
            }

            pub fn flags_for(&self, scope: &str) -> Option<&[FeatureFlag]> {
                self.entries.get(scope).map(|feats| feats.as_slice())
            }

            pub fn scopes(&self) -> impl Iterator<Item = (&str, &[FeatureFlag])> {
                self.entries.iter().map(|(scope, ff)| (scope.as_str(), ff.as_slice()))
            }
        }

        #[cfg(feature = "preview")]
        pub fn render_preview(registry: &FeatureRegistry) -> String {
            let mut lines = Vec::<String>::with_capacity(registry.entries.len());
            for (scope, ffs) in registry.scopes() {
                for ff in ffs {
                    lines.push(format!("{}::{} -> {}", scope, ff.name, ff.state.label()));
                }
            }
            if lines.is_empty() {
                return String::from("");
            }
            let mut lines = lines.join("\n");
            lines.push('\n');
            lines
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
