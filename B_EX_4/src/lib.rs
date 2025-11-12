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
    use std::io::{BufRead as _, BufReader};
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
            match raw.to_lowercase().trim() {
                "pass" => Ok(TestOutcome::Passed),
                "fail" => Ok(TestOutcome::Failed),
                "skip" => Ok(TestOutcome::Skipped),
                _ => Err(ReportError::InvalidOutcome{ raw: raw.to_string() }),
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
            let data = line.split('|')
                .map(|s| s.trim())
                .collect::<Vec<&str>>();
            if data.len() != 3 {
                return Err(ReportError::InvalidFormat { line: line.to_string() });
            }
            let (suite, case) = data[0].split_once("::")
                .ok_or(ReportError::InvalidFormat { line: line.to_string() })?;
            let outcome:TestOutcome = TestOutcome::from_str(data[1])?;
            let dur = data[2];
            let duration_ms: u64 =  dur.parse()
                .map_err(|_| ReportError::InvalidDuration { raw: dur.to_string() })?;

            Ok(TestCase { suite: suite.to_string(), case: case.to_string(), outcome, duration_ms })
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
                ReportError::InvalidFormat{line} => write!(f, "Niepoprawny format linii: {line}"),
                ReportError::InvalidOutcome{raw} => write!(f, "Nieznany wynik testu: {raw}"),
                ReportError::InvalidDuration { raw} => write!(f, "Niepoprawny czas trwania: {raw}"),
                ReportError::NoCases => write!(f, "Brak przypadków testowych")
            }
        }
    }

    impl std::error::Error for ReportError {}

    /// Parsuje wpisy testów z bufora.
    pub fn parse_cases<R: BufRead>(reader: R) -> Result<Vec<TestCase>, ReportError> {
        let mut cases_list: Vec<TestCase> = Vec::new();
        for line_result in reader.lines() {
            let line = line_result.map_err(|s| ReportError::InvalidFormat { line: s.to_string() })?;
            if line.is_empty() || line.starts_with('#') { continue; }

            let case = TestCase::from_line(line.as_str())?;
            cases_list.push(case);
        }

        if cases_list.is_empty() {
            return Err(ReportError::NoCases);
        }

        Ok(cases_list)
    }

    /// Agreguje przypadki według pakietu.
    pub fn summarize_by_suite(cases: &[TestCase]) -> Vec<SuiteSummary> {
        let mut summaries: BTreeMap<String, SuiteSummary> = BTreeMap::new();
        for case in cases {
            let entry = summaries.entry(case.suite.clone())
                .or_insert_with(|| SuiteSummary {
                    suite: case.suite.clone(),
                    total: 0,
                    passed: 0,
                    failed: 0,
                    skipped: 0,
                    total_duration_ms: 0,
                });
            entry.total += 1;
            entry.total_duration_ms += case.duration_ms;
            match case.outcome {
                TestOutcome::Passed => entry.passed += 1,
                TestOutcome::Failed => entry.failed += 1,
                TestOutcome::Skipped => entry.skipped += 1,
            }
        }
        summaries.into_values().collect()
    }

    /// Formatuje podsumowanie w linie tekstowe.
    pub fn format_summary(summaries: &[SuiteSummary]) -> Vec<String> {
        let mut formatted_summaries: Vec<String> = Vec::new();

        for summary in summaries {
            let case_word = if summary.total == 1 { "przypadek" } else { "przypadki" };
            let formatted_summary = format!("Suite {}: {} {} (pass: {}, fail: {}, skip: {}) - łączny czas {}ms", summary.suite, summary.total, case_word, summary.passed, summary.failed, summary.skipped, summary.total_duration_ms);
            formatted_summaries.push(formatted_summary);
        }
        formatted_summaries
    }

    /// Tworzy kompletny raport z wejścia.
    pub fn collect_report<R: BufRead>(reader: R) -> Result<Vec<String>, ReportError> {
        Ok(format_summary(&(summarize_by_suite(&parse_cases(reader)?))))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::io::Cursor;

        #[test]
        fn poprawnie_parsuje_pojedynczy_wpis() {
            let line = "Checkout::Test1 | pass | 100 ";
            let case = TestCase::from_line(line).unwrap();
            assert_eq!(case.suite, "Checkout");
            assert_eq!(case.case, "Test1");
            assert_eq!(case.outcome, TestOutcome::Passed);
            assert_eq!(case.duration_ms, 100);
        }

        #[test]
        fn agreguje_statystyki_dla_pakietu() {
            let cases = vec![
                TestCase::from_line("SuiteA::Test1 | pass | 100 ").unwrap(),
                TestCase::from_line("SuiteB::Test1 | skip | 50 ").unwrap(),
                TestCase::from_line("SuiteA::Test2 | fail | 200 ").unwrap(),
            ];
            let summarized = summarize_by_suite(&cases);
            assert_eq!(summarized.len(), 2);

            let summary_a = summarized.iter().find(|s| s.suite == "SuiteA").unwrap();
            assert_eq!(summary_a.suite, "SuiteA");
            assert_eq!(summary_a.total, 2);
            assert_eq!(summary_a.passed, 1);
            assert_eq!(summary_a.failed, 1);
            assert_eq!(summary_a.skipped, 0);
            assert_eq!(summary_a.total_duration_ms, 300);

            let summary_b = summarized.iter().find(|s| s.suite == "SuiteB").unwrap();
            assert_eq!(summary_b.suite, "SuiteB");
            assert_eq!(summary_b.total, 1);
            assert_eq!(summary_b.passed, 0);
            assert_eq!(summary_b.failed, 0);
            assert_eq!(summary_b.skipped, 1);
            assert_eq!(summary_b.total_duration_ms, 50);
        }

        #[test]
        fn collect_report_summarizes_suites() {
            let lines =
                "SuiteA::Test1 | pass | 100 \nSuiteB::Test1 | skip | 50 \nSuiteA::Test2 | fail | 200 ";
            let reader = Cursor::new(lines);
            let expected_result = vec![
                "Suite SuiteA: 2 przypadki (pass: 1, fail: 1, skip: 0) - łączny czas 300ms",
                "Suite SuiteB: 1 przypadek (pass: 0, fail: 0, skip: 1) - łączny czas 50ms"
            ];
            let actual_result = collect_report(reader).unwrap();
            assert_eq!(actual_result, expected_result);
        }

        #[test]
        fn requires_non_null_input() {
            let reader = Cursor::new("#xd");
            assert!(collect_report(reader).is_err());
        }

        #[test]
        fn invalid_duration_reported() {
            let lines =
                "SuiteA::Test1 | pass | tududu ";
            let reader = Cursor::new(lines);

            assert!(collect_report(reader).is_err());
        }

        #[test]
        fn invalid_duration_is_reported() {
            let error = TestCase::from_line("Checkout::smoke | pass | NaN")
                .expect_err("invalid duration should fail");

            assert_eq!(error.to_string(), "Niepoprawny czas trwania: NaN");
            assert!(matches!(error, ReportError::InvalidDuration { raw } if raw == "NaN"));
        }
    }
}