use std::fmt;
use std::io::BufRead;

pub use report::{
    collect_report, format_summary, parse_cases, summarize_by_suite, ReportError, SuiteSummary,
    TestCase, TestOutcome,
};

/// Logika raportu z testów regresyjnych.
pub mod report {
    use super::BufRead;
    use super::fmt;
    use std::io::BufRead as _;
    use std::str::FromStr;

    /// Możliwe wyniki pojedynczego testu.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum TestOutcome {
        Passed,
        Failed,
        Skipped,
    }

    impl TestOutcome {
        /// Nazwa wyświetlana w raporcie.
        pub fn label(&self) -> &'static str {
            todo!("zwróć skróconą nazwę wyniku: pass/fail/skip")
        }
    }

    impl FromStr for TestOutcome {
        type Err = ReportError;

        fn from_str(raw: &str) -> Result<Self, Self::Err> {
            todo!("parsuj wyniki pass/fail/skip niezależnie od wielkości liter")
        }
    }

    /// Pojedynczy wpis raportu testów.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TestCase {
        pub suite: String,
        pub case: String,
        pub outcome: TestOutcome,
        pub duration_ms: u64,
    }

    impl TestCase {
        /// Tworzy wpis na podstawie linii `suite::case | outcome | duration_ms`.
        pub fn from_line(line: &str) -> Result<Self, ReportError> {
            todo!("rozbij linię, zweryfikuj format i przygotuj strukturę TestCase")
        }
    }

    /// Zbiorcze statystyki dla pakietu testów.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SuiteSummary {
        pub suite: String,
        pub total: usize,
        pub passed: usize,
        pub failed: usize,
        pub skipped: usize,
        pub total_duration_ms: u64,
    }

    /// Błędy parsowania i agregacji raportu.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ReportError {
        InvalidFormat { line: String },
        InvalidOutcome { raw: String },
        InvalidDuration { raw: String },
        NoCases,
    }

    impl fmt::Display for ReportError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            todo!("zamień warianty błędów na czytelne komunikaty")
        }
    }

    impl std::error::Error for ReportError {}

    /// Parsuje wpisy testów z bufora.
    pub fn parse_cases<R: BufRead>(reader: R) -> Result<Vec<TestCase>, ReportError> {
        todo!("iteruj po liniach, pomijaj komentarze i puste linie, użyj TestCase::from_line")
    }

    /// Agreguje przypadki według pakietu.
    pub fn summarize_by_suite(cases: &[TestCase]) -> Vec<SuiteSummary> {
        todo!("zlicz testy dla każdego suite i zwróć podsumowanie w kolejności alfabetycznej")
    }

    /// Formatuje podsumowanie w linie tekstowe.
    pub fn format_summary(summaries: &[SuiteSummary]) -> Vec<String> {
        todo!("przygotuj opis pakietów zgodnie z wymaganym formatem")
    }

    /// Tworzy kompletny raport z wejścia.
    pub fn collect_report<R: BufRead>(reader: R) -> Result<Vec<String>, ReportError> {
        todo!("połącz parse_cases, summarize_by_suite i format_summary, propagując błędy operatorem ?")
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn poprawnie_parsuje_pojedynczy_wpis() {
            todo!("dodaj test parsowania zgodnie z instrukcją w README")
        }

        #[test]
        fn agreguje_statystyki_dla_pakietu() {
            todo!("dodaj test, który sprawdza liczniki i czas dla summarize_by_suite")
        }
    }
}
