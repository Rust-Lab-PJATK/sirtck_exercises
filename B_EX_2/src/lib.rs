use std::collections::BTreeMap;
use std::io::{BufRead, Read};
use std::str::FromStr;

pub mod domain {
    //! Domain types and parsing logic for individual task records.
    use std::fmt;
    use std::str::FromStr;

    /// Task priority listed from the highest to the lowest.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Priority {
        High,
        Medium,
        Low,
    }

    impl Priority {
        /// Ordering used when reporting (`high`, `medium`, `low`).
        pub fn all() -> [Priority; 3] {
            [Priority::High, Priority::Medium, Priority::Low]
        }

        /// Label shown in the report (`high`, `medium`, `low`).
        pub fn label(self) -> &'static str {
            match self {
                Priority::High => "high",
                Priority::Medium => "medium",
                Priority::Low => "low",
            }
        }
    }

    impl fmt::Display for Priority {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.label())
        }
    }

    /// Task status in business order: TODO -> IN_PROGRESS -> DONE.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Status {
        Todo,
        InProgress,
        Done,
    }

    impl Status {
        /// All statuses in the order they must appear in the report.
        pub fn all() -> [Status; 3] {
            [Status::Todo, Status::InProgress, Status::Done]
        }

        /// Label used when printing to stdout (e.g. `TODO`, `IN_PROGRESS`, `DONE`).
        pub fn label(self) -> &'static str {
            match self {
                Status::Todo => "TODO",
                Status::InProgress => "IN_PROGRESS",
                Status::Done => "DONE",
            }
        }
    }

    impl fmt::Display for Status {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.label())
        }
    }

    /// Parsing error for a single input line.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TaskParseError {
        /// No valid tasks were provided.
        NoTasks,
        /// Line does not have exactly three fields split by `|`.
        InvalidFormat(String),
        /// Unknown priority (middle field).
        InvalidPriority(String),
        /// Unknown status (last field).
        InvalidStatus(String),
    }

    impl fmt::Display for TaskParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TaskParseError::NoTasks => write!(f, "Brak zadan"),
                TaskParseError::InvalidFormat(s) => write!(f, "Niepoprawny format linii: {}", s),
                TaskParseError::InvalidPriority(s) => write!(f, "Nieznany priorytet: {}", s),
                TaskParseError::InvalidStatus(s) => write!(f, "Nieznany status: {}", s),
            }
        }
    }

    impl std::error::Error for TaskParseError {}

    /// Single task record described in the input.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Task {
        pub title: String,
        pub priority: Priority,
        pub status: Status,
    }

    impl FromStr for Task {
        type Err = TaskParseError;

        /// Parses a `title | priority | status` line.
        fn from_str(line: &str) -> Result<Self, Self::Err> {
            if line.is_empty() {
                return Err(TaskParseError::NoTasks);
            }
            let words  = line.trim()
                .split("|")
                .map(|word| word.trim())
                .collect::<Vec<&str>>();
            if words.len() != 3 {
                return Err(TaskParseError::InvalidFormat(line.to_string()));
            }

            let priority_str = words[1].to_ascii_lowercase();

            let priorities = Priority::all();
            let priority = priorities
                .iter()
                .find(|p| p.label() == priority_str)
                .ok_or_else(|| TaskParseError::InvalidPriority(priority_str.clone()))?;

            let status_str = words[2].to_ascii_uppercase();
            let statuses = Status::all();
            let status = statuses.iter()
                .find(|s| s.label() == status_str)
                .ok_or_else(|| TaskParseError::InvalidStatus(status_str.clone()))?;

            let title = words[0].to_string();
            Ok(Task{title, priority: *priority, status: *status})
        }
    }
}

use domain::{Priority, Status, Task, TaskParseError};

/// Aggregated data for a specific status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusSummary {
    pub status: Status,
    pub counts: BTreeMap<Priority, usize>,
}

impl StatusSummary {
    /// Returns the number of tasks in this status.
    pub fn total(&self) -> usize {
        self.counts.values().sum()
    }
}

/// Parses multiple tasks from raw text, skipping empty lines.
///
/// At least one valid task must be present or [`TaskParseError::NoTasks`] is returned.
pub fn parse_tasks(input: &str) -> Result<Vec<Task>, TaskParseError> {
    let tasks = input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| Task::from_str(s))
        .collect::<Result<Vec<_>, _>>()?;

    if tasks.is_empty() {
        Err(TaskParseError::NoTasks)
    } else {
        Ok(tasks)
    }
}

/// Builds summaries for every status in the required order.
///
/// The result must contain entries for all statuses (`TODO`, `IN_PROGRESS`, `DONE`) even when the
/// total count is zero. Each entry keeps per-priority counters within a `BTreeMap`.
pub fn summarize_by_status(tasks: &[Task]) -> Vec<StatusSummary> {
    Status::all()
        .into_iter()
        .map(|status| {
            let counts = Priority::all()
                .into_iter()
                .map(|priority| {
                    let count = tasks
                        .iter()
                        .filter(|t| t.status == status && t.priority == priority)
                        .count();
                    (priority, count)
                }).collect::<BTreeMap<_, _>>();
            StatusSummary{status, counts}
        })
    .collect()
}

/// Formats aggregated data ready to be displayed.
///
/// Every line should follow the pattern
/// `TODO: 2 (high: 1, medium: 1, low: 0)`
/// while keeping priority order `high`, `medium`, `low`.
pub fn format_summary(summary: &[StatusSummary]) -> Vec<String> {
    summary
        .iter()
        .map(|summary| {
            let get = |it| summary.counts.get(&it).copied().unwrap_or(0);
            format!(
                "{}: {} (high: {}, medium: {}, low: {})",
                summary.status,
                summary.total(),
                get(Priority::High),
                get(Priority::Medium),
                get(Priority::Low)
            )
        })
        .collect()
}

/// Full pipeline: read, parse, aggregate and format.
pub fn run_from_reader<R: BufRead>(reader: R) -> Result<Vec<String>, TaskParseError> {
    use std::io;
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).expect(TaskParseError::NoTasks.to_string().as_str());
    
    let parsed = parse_tasks(&buffer)?;
    let status_summary = summarize_by_status(&parsed);
    Ok(format_summary(&status_summary))
}

/// Convenience API for tests that accepts input as a single string.
pub fn run_from_str(input: &str) -> Result<Vec<String>, TaskParseError> {
    let cursor = std::io::Cursor::new(input.as_bytes());
    run_from_reader(cursor)
}
