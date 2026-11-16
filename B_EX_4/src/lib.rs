use std::fmt;
use std::io::BufRead;

pub use report::{
    collect_report, format_summary, parse_cases, summarize_by_suite, ReportError, SuiteSummary,
    TestCase, TestOutcome,
};

/// Logika raportu z testów regresyjnych.
pub mod report {
    use std::collections::BTreeMap;
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
            match self {
                TestOutcome::Passed => "pass",
                TestOutcome::Failed => "fail",
                TestOutcome::Skipped => "skip"
            }
        }
    }

    impl FromStr for TestOutcome {
        type Err = ReportError;

        fn from_str(raw: &str) -> Result<Self, Self::Err> {
            let lowercase = raw.to_lowercase();
            match lowercase.as_str() {
                "pass" => Ok(TestOutcome::Passed),
                "fail" => Ok(TestOutcome::Failed),
                "skip" => Ok(TestOutcome::Skipped),
                _ => Err(ReportError::InvalidOutcome { raw: raw.to_string() })
            }
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
            let parts: Vec<&str> = line.split("|").map(|part| part.trim()).collect();
            if parts.len() != 3 {
                return Err(ReportError::InvalidFormat { line: line.to_string() })
            }

            let (suite, case) = parts[0]
                .split_once("::")
                .map(|(fist_val, second_val) | (fist_val.trim().to_string(), second_val.trim().to_string()))
                .ok_or_else(|| ReportError::InvalidFormat { line: line.to_string() })?;

            let outcome = TestOutcome::from_str(parts[1])?;

            let duration_raw_string = parts[2];
            let duration_ms : u64 = duration_raw_string.parse().map_err(|_| ReportError::InvalidDuration { raw: duration_raw_string.to_string() })?;

            Ok(TestCase { suite, case, outcome, duration_ms })
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
            match self {
                ReportError::InvalidFormat { line } => write!(f, "Niepoprawny format linii: {}", line),
                ReportError::InvalidOutcome { raw } => write!(f, "Nieznany wynik testu: {}", raw),
                ReportError::InvalidDuration { raw } => write!(f, "Niepoprawny czas trwania: {}", raw),
                ReportError::NoCases => write!(f, "Brak przypadków testowych")
            }
        }
    }

    impl std::error::Error for ReportError {}

    /// Parsuje wpisy testów z bufora.
    pub fn parse_cases<R: BufRead>(reader: R) -> Result<Vec<TestCase>, ReportError> {
        let mut results: Vec<TestCase> = vec!();
        for line in reader.lines() {
            let line = line.unwrap();
            if line.trim().is_empty() || line.trim().starts_with("#") {
                continue;
            }
            results.push(TestCase::from_line(line.as_str())?)
        }
        if results.is_empty() {
            Err(ReportError::NoCases)
        } else {
            Ok(results)
        }
    }

    /// Agreguje przypadki według pakietu.
    pub fn summarize_by_suite(cases: &[TestCase]) -> Vec<SuiteSummary> {
        let mut cases_by_suites: BTreeMap<String, SuiteSummary> = BTreeMap::new();
        for case in cases {
            let map_entry = cases_by_suites.entry(case.suite.clone()).or_insert_with(||
                SuiteSummary {
                    suite: case.suite.clone(),
                    total: 0,
                    passed: 0,
                    failed: 0,
                    skipped: 0,
                    total_duration_ms: 0,
                }
            );
            map_entry.total += 1;

            match case.outcome {
                TestOutcome::Passed => map_entry.passed += 1,
                TestOutcome::Failed => map_entry.failed += 1,
                TestOutcome::Skipped => map_entry.skipped += 1
            }

            map_entry.total_duration_ms += case.duration_ms;
        }
        cases_by_suites.into_values().collect()
    }

    /// Formatuje podsumowanie w linie tekstowe.
    pub fn format_summary(summaries: &[SuiteSummary]) -> Vec<String> {
        summaries.iter().map(|summary|
            format!("Suite {}: {} przypadki (pass: {}, fail: {}, skip: {}) - łączny czas {}ms",
                    summary.suite, summary.total, summary.passed, summary.failed, summary.skipped, summary.total_duration_ms
            )
        ).collect()
    }

    /// Tworzy kompletny raport z wejścia.
    pub fn collect_report<R: BufRead>(reader: R) -> Result<Vec<String>, ReportError> {
        let cases = parse_cases(reader)?;
        let cases_by_suite = summarize_by_suite(cases.as_slice());
        Ok(format_summary(cases_by_suite.as_slice()))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn poprawnie_parsuje_pojedynczy_wpis() {
            assert_eq!(
                TestCase::from_line("checkout::card_payment   |pass|230"),
                Ok(TestCase {
                    suite: "checkout".to_string(),
                    case: "card_payment".to_string(),
                    outcome: TestOutcome::Passed,
                    duration_ms: 230,
                })
            )
        }

        #[test]
        fn odrzuca_przypadek_z_nieznanym_wynikiem() {
            assert!(matches!(
                TestCase::from_line("checkout::card_payment   |passed|230"),
                Err(ReportError::InvalidOutcome { .. })
            ));
        }

        #[test]
        fn odrzuca_przypadek_z_niepoprawnym_formatem_linii() {
            assert!(matches!(
                TestCase::from_line("checkout:card_payment   |pass|230"),
                Err(ReportError::InvalidFormat { .. })
            ));
        }
        #[test]
        fn odrzuca_przypadek_z_niepoprawna_liczba_argumentow() {
            assert!(matches!(
                TestCase::from_line("checkout:card_payment   |pass"),
                Err(ReportError::InvalidFormat { .. })
            ));
        }

        #[test]
        fn odrzuca_przypadek_z_niepoprawnym_czasem_trwania() {
            assert!(matches!(
                TestCase::from_line("checkout::card_payment   |pass|230ms"),
                Err(ReportError::InvalidDuration { .. })
            ));
        }

        #[test]
        fn agreguje_statystyki_dla_pakietu() {
            assert_eq!(
                summarize_by_suite(&[
                    TestCase::from_line("checkout::card_payment   |pass|230").unwrap(),
                    TestCase::from_line("checkout::wrong_card_credentials   |fail|230").unwrap(),
                    TestCase::from_line("login::authentication_successful   |skip|100").unwrap(),
                ]),
                vec!(
                    SuiteSummary {
                        suite: "checkout".to_string(),
                        total: 2,
                        passed: 1,
                        failed: 1,
                        skipped: 0,
                        total_duration_ms: 460,
                    },
                    SuiteSummary {
                        suite: "login".to_string(),
                        total: 1,
                        passed: 0,
                        failed: 0,
                        skipped: 1,
                        total_duration_ms: 100,
                    },
                )
            )
        }
    }
}
