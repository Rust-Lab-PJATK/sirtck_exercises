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
    use std::collections::BTreeMap;
    // use std::io::BufRead as _;
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
                TestOutcome::Skipped => "skip",
            }
        }
    }

    impl FromStr for TestOutcome {
        type Err = ReportError;

        fn from_str(raw: &str) -> Result<Self, Self::Err> {
            match raw.to_lowercase().as_str() {
                "pass" => Ok(TestOutcome::Passed),
                "fail" => Ok(TestOutcome::Failed),
                "skip" => Ok(TestOutcome::Skipped),
                _ => Err(ReportError::InvalidOutcome { raw: raw.to_string() }),
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
            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() != 3 {
                return Err(ReportError::InvalidFormat { line: line.to_string() });
            }

            let raw_suite = parts[0].trim();
            let raw_outcome = parts[1].trim();
            let raw_duration = parts[2].trim();

            let (suite, case) = raw_suite.split_once("::").ok_or_else(|| {
                ReportError::InvalidFormat {
                    line: line.to_string(),
                }
            })?;

            let outcome = TestOutcome::from_str(raw_outcome)?;
            let duration_ms = raw_duration.parse::<u64>().map_err(|_| {
                ReportError::InvalidDuration {
                    raw: raw_duration.to_string(),
                }
            })?;

            Ok(TestCase {
                suite: suite.to_string(),
                case: case.to_string(),
                outcome,
                duration_ms,
            })
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
                ReportError::InvalidFormat { line } => {
                    write!(f, "Niepoprawny format linii: '{}'", line)
                }
                ReportError::InvalidOutcome { raw } => {
                    write!(f, "Nieznany wynik testu: '{}'", raw)
                }
                ReportError::InvalidDuration { raw } => {
                    write!(f, "Niepoprawny czas trwania: '{}'", raw)
                }
                ReportError::NoCases => write!(f, "Brak przypadków testowych"),
            }
        }
    }

    impl std::error::Error for ReportError {}

    /// Parsuje wpisy testów z bufora.
    pub fn parse_cases<R: BufRead>(reader: R) -> Result<Vec<TestCase>, ReportError> {
        let mut cases = Vec::new();
        for line in reader.lines() {
            let line = line.map_err(|_| ReportError::NoCases)?;
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("#") {
                continue;
            }
            cases.push(TestCase::from_line(trimmed)?);
        }
        if cases.is_empty() {
            return Err(ReportError::NoCases);
        }
        Ok(cases)
    }

    /// Agreguje przypadki według pakietu.
    pub fn summarize_by_suite(cases: &[TestCase]) -> Vec<SuiteSummary> {
        let mut summaries: BTreeMap<String, SuiteSummary> = BTreeMap::new();
        for case in cases {
            let summary = summaries.entry(case.suite.clone()).or_insert(SuiteSummary {
                suite: case.suite.clone(),
                total: 0,
                passed: 0,
                failed: 0,
                skipped: 0,
                total_duration_ms: 0,
            });

            summary.total += 1;
            summary.total_duration_ms += case.duration_ms;
            match case.outcome {
                TestOutcome::Passed => summary.passed += 1,
                TestOutcome::Failed => summary.failed += 1,
                TestOutcome::Skipped => summary.skipped += 1,
            }
        }
        summaries.into_values().collect()
    }

    /// Formatuje podsumowanie w linie tekstowe.
    pub fn format_summary(summaries: &[SuiteSummary]) -> Vec<String> {
        summaries.iter().map(|s|{
            format!(
                "Suite {}: {} przypadki (pass: {}, fail: {}, skip: {}) - łączny czas {}ms",
                s.suite, s.total, s.passed, s.failed, s.skipped, s.total_duration_ms
            )
        }).collect()
    }

    /// Tworzy kompletny raport z wejścia.
    pub fn collect_report<R: BufRead>(reader: R) -> Result<Vec<String>, ReportError> {
        let cases = parse_cases(reader)?;
        let summaries = summarize_by_suite(&cases);
        Ok(format_summary(&summaries))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn poprawnie_parsuje_pojedynczy_wpis() {
            let line = "Auth::login | pass | 120";
            let case = TestCase::from_line(line).unwrap();
            assert_eq!(case.suite, "Auth");
            assert_eq!(case.case, "login");
            assert_eq!(case.outcome, TestOutcome::Passed);
            assert_eq!(case.duration_ms, 120);
        }

        #[test]
        fn agreguje_statystyki_dla_pakietu() {
            let cases = vec![
                TestCase {
                    suite: "Auth".to_string(),
                    case: "login".to_string(),
                    outcome: TestOutcome::Passed,
                    duration_ms: 120,
                },
                TestCase {
                    suite: "Auth".to_string(),
                    case: "logout".to_string(),
                    outcome: TestOutcome::Passed,
                    duration_ms: 80,
                },
                TestCase {
                    suite: "Checkout".to_string(),
                    case: "add_item".to_string(),
                    outcome: TestOutcome::Failed,
                    duration_ms: 250,
                },
            ];
            let summaries = summarize_by_suite(&cases);

            assert_eq!(summaries.len(), 2);
            assert_eq!(summaries[0].suite, "Auth");
            assert_eq!(summaries[0].total, 2);
            assert_eq!(summaries[0].passed, 2);
            assert_eq!(summaries[0].failed, 0);
            assert_eq!(summaries[0].total_duration_ms, 200);

            assert_eq!(summaries[1].suite, "Checkout");
            assert_eq!(summaries[1].total, 1);
            assert_eq!(summaries[1].failed, 1);
            assert_eq!(summaries[1].total_duration_ms, 250);
        }
    }
}
