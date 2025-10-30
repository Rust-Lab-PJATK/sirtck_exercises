//! Rdzeń zadania RC_EX_3 – deklaratywny opis zdarzeń telemetryjnych.

pub use rc_ex_3_macros::EventBlueprint;

/// Oznacza, czy pole jest obowiązkowe, czy może zostać pominięte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    Required,
    Optional,
}

/// Metadane pojedynczego pola.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldSpec {
    pub name: &'static str,
    pub ty: &'static str,
    pub kind: FieldKind,
}

impl FieldSpec {
    /// Stała konstrukcja, żeby makra mogły produkować `FieldSpec`.
    pub const fn new(name: &'static str, ty: &'static str, kind: FieldKind) -> Self {
        Self { name, ty, kind }
    }
}

/// Pełny opis zdarzenia telemetryjnego.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventSpec {
    pub name: &'static str,
    pub fields: &'static [FieldSpec],
}

impl EventSpec {
    /// Łatwiejsza w użyciu konstanta konstrukcja.
    pub const fn new(name: &'static str, fields: &'static [FieldSpec]) -> Self {
        Self { name, fields }
    }
}

/// Trait udostępniany przez derive-makro.
pub trait EventBlueprint {
    /// Zwraca statyczny opis zdarzenia.
    fn blueprint() -> &'static EventSpec;
}

#[macro_export]
macro_rules! event_fields {
    ($($tt:tt)*) => {{
        compile_error!("Zaimplementuj makro event_fields!");
    }};
}

#[macro_export]
macro_rules! event_spec {
    ($($tt:tt)*) => {{
        compile_error!("Zaimplementuj makro event_spec!");
    }};
}

pub use crate::{event_fields, event_spec};
