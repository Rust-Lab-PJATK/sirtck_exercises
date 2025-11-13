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
                todo!("zwróć napis opisujący stan, np. enabled/disabled/percentage:25")
            }
        }

        impl FromStr for FeatureState {
            type Err = ConfigError;

            fn from_str(raw: &str) -> Result<Self, Self::Err> {
                todo!("parsuj literały enabled/disabled/percentage:<0-100>")
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
                todo!("rozbij obszar i nazwę flagi oraz sparsuj stan")
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
                todo!("dopasuj warianty błędów do opisów z README")
            }
        }

        impl std::error::Error for ConfigError {}
    }

    pub mod parser {
        use super::model::{ConfigError, FeatureFlag};
        use crate::FeatureRegistry;
        use std::io::BufRead;

        pub fn parse_flags<R: BufRead>(reader: R) -> Result<Vec<FeatureFlag>, ConfigError> {
            todo!("wczytaj linie z BufRead, pomijaj komentarze i puste wiersze")
        }

        pub fn load_registry<R: BufRead>(reader: R) -> Result<FeatureRegistry, ConfigError> {
            todo!("zbuduj FeatureRegistry na podstawie parse_flags")
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
                todo!("utwórz rejestr i dodaj wszystkie flagi, propagując błędy insert")
            }

            pub fn insert(&mut self, flag: FeatureFlag) -> Result<(), ConfigError> {
                todo!("dodaj flagę, dbając o kolejność i brak duplikatów")
            }

            pub fn flags_for(&self, scope: &str) -> Option<&[FeatureFlag]> {
                todo!("zwróć wycinek flag dla wskazanego obszaru")
            }

            pub fn scopes(&self) -> impl Iterator<Item = (&str, &[FeatureFlag])> {
                todo!("zwróć iterator po parach (obszar, flagi[])")
            }
        }

        #[cfg(feature = "preview")]
        pub fn render_preview(registry: &FeatureRegistry) -> String {
            todo!("przygotuj podgląd rozmieszczonych flag z etykietami")
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
