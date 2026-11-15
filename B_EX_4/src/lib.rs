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
            let lower = raw.to_ascii_lowercase();
            let lower = lower.as_str();
            match lower {
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
            let tokens = line.split(" | ").collect::<Vec<_>>();
            if tokens.len() != 3 {
                return Err(ReportError::InvalidFormat { line: line.to_string() });
            }
            let (suite, case) = match tokens[0].split_once("::") {
                Some(pair) => pair,
                None => return Err(ReportError::InvalidFormat { line: line.to_string() })
            };
            let suite = suite.trim().to_string();
            let case = case.trim().to_string();
            let outcome = tokens[1].trim();
            let duration_ms = tokens[2].trim();

            let outcome = TestOutcome::from_str(outcome)?;
            let duration_ms = duration_ms.parse::<u64>().map_err(|e| ReportError::InvalidDuration { raw: duration_ms.to_string() })?;

            Ok(TestCase {
                suite,
                case,
                outcome,
                duration_ms
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
                ReportError::InvalidFormat { line } => write!(f, "Invalid format: {}", line),
                ReportError::InvalidOutcome { raw } => write!(f, "Invalid outcome: {}", raw),
                ReportError::InvalidDuration { raw } => write!(f, "Invalid duration: {}", raw),
                ReportError::NoCases => write!(f, "No cases to report")
            }
        }
    }

    impl std::error::Error for ReportError {}

    /// Parsuje wpisy testów z bufora.
    pub fn parse_cases<R: BufRead>(reader: R) -> Result<Vec<TestCase>, ReportError> {
        let mut cases = Vec::<TestCase>::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let case = TestCase::from_line(&line)?;
            cases.push(case);
        }
        if cases.is_empty() {
            return Err(ReportError::NoCases);
        }
        Ok(cases)
    }

    /// Agreguje przypadki według pakietu.
    pub fn summarize_by_suite(cases: &[TestCase]) -> Vec<SuiteSummary> {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::<&str, SuiteSummary>::new();
        for case in cases {
            let summary = map.entry(case.suite.as_str()).or_insert(SuiteSummary {
                suite: case.suite.to_string(),
                total: 0,
                passed: 0,
                failed: 0,
                skipped: 0,
                total_duration_ms: 0
            });
            let outcome_count = match case.outcome {
                TestOutcome::Passed => &mut summary.passed,
                TestOutcome::Failed => &mut summary.failed,
                TestOutcome::Skipped => &mut summary.skipped
            };
            *outcome_count += 1;
            summary.total += 1;
            summary.total_duration_ms += case.duration_ms;
        }
        let summaries = map.into_values().collect::<Vec<_>>();
        summaries
    }

    /// Formatuje podsumowanie w linie tekstowe.
    pub fn format_summary(summaries: &[SuiteSummary]) -> Vec<String> {
        let mut lines = Vec::<String>::new();
        for summary in summaries {
            let suffix = match summary.total {
                0 => "ków",
                1 => "ek",
                2..=4 => "ki",
                _ => "ków"
            };
            let line = format!(
                "{} Checkout: {} przypad{} ({}: {}, {}: {}, {}: {}) - łączny czas {}ms",
                summary.suite, summary.total, suffix, 
                TestOutcome::Passed.label(), summary.passed, 
                TestOutcome::Failed.label(), summary.failed, 
                TestOutcome::Skipped.label(), summary.skipped, 
                summary.total_duration_ms);
            lines.push(line);
        }
        lines
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
            let input = "mocksuite::mockcase | pass | 100";
            let case = TestCase::from_line(input).unwrap();
            assert!(matches!(case.suite.as_str(), "mocksuite"));
            assert!(matches!(case.case.as_str(), "mockcase"));
            assert!(matches!(case.outcome, TestOutcome::Passed));
            assert!(matches!(case.duration_ms, 100));
        }

        #[test]
        fn agreguje_statystyki_dla_pakietu() {
            let cases = vec![
                TestCase {
                    suite: "suite-a".to_string(),
                    case: "case-a1".to_string(),
                    outcome: TestOutcome::Passed,
                    duration_ms: 10
                },
                TestCase {
                    suite: "suite-a".to_string(),
                    case: "case-a2".to_string(),
                    outcome: TestOutcome::Failed,
                    duration_ms: 20
                },
                TestCase {
                    suite: "suite-b".to_string(),
                    case: "case-b1".to_string(),
                    outcome: TestOutcome::Skipped,
                    duration_ms: 30
                }
            ];

            let summaries = summarize_by_suite(&cases);
            assert!(matches!(summaries.len(), 2));

            let suite_a = &summaries[0];
            assert!(matches!(suite_a.suite.as_str(), "suite-a"));
            assert!(matches!(suite_a.total, 2));
            assert!(matches!(suite_a.passed, 1));
            assert!(matches!(suite_a.failed, 1));
            assert!(matches!(suite_a.skipped, 0));
            assert!(matches!(suite_a.total_duration_ms, 30));

            let suite_b = &summaries[1];
            assert!(matches!(suite_b.suite.as_str(), "suite-b"));
            assert!(matches!(suite_b.total, 1));
            assert!(matches!(suite_b.passed, 0));
            assert!(matches!(suite_b.failed, 0));
            assert!(matches!(suite_b.skipped, 1));
            assert!(matches!(suite_b.total_duration_ms, 30));
        }
    }
}
