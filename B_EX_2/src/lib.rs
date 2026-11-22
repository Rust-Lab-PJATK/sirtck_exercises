use std::collections::BTreeMap;
use std::io::BufRead;
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
            f.write_str(self.label())
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
            f.write_str(self.label())
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
                TaskParseError::NoTasks => f.write_str("Brak zadan"),
                TaskParseError::InvalidFormat(x) => {
                    f.write_str(&format!("Niepoprawny format linii: {x}"))
                }
                TaskParseError::InvalidPriority(x) => {
                    f.write_str(&format!("Nieznany priorytet: {x}"))
                }
                TaskParseError::InvalidStatus(x) => f.write_str(&format!("Nieznany status: {x}")),
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
            let parts: Vec<&str> = line.split("|").collect();
            if parts.len() != 3 {
                return Err(TaskParseError::InvalidFormat(line.to_string()));
            }

            let title = parts[0].trim().to_string();
            let priority_str = parts[1].trim();
            let status_str = parts[2].trim();
            let priority = match priority_str.to_lowercase().as_str() {
                "high" => Priority::High,
                "medium" => Priority::Medium,
                "low" => Priority::Low,
                _ => return Err(TaskParseError::InvalidPriority(priority_str.to_string())),
            };
            let status = match status_str.to_lowercase().as_str() {
                "todo" => Status::Todo,
                "in_progress" => Status::InProgress,
                "done" => Status::Done,
                _ => return Err(TaskParseError::InvalidStatus(status_str.to_string())),
            };
            Ok(Task {
                title,
                priority,
                status,
            })
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
    let tasks: Result<Vec<Task>, TaskParseError> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| Task::from_str(line))
        .collect();
    let tasks_ = match tasks {
        Ok(x) => x,
        Err(x) => Err(x)?,
    };

    if tasks_.is_empty() {
        return Err(TaskParseError::NoTasks);
    }
    Ok(tasks_)
}

/// Builds summaries for every status in the required order.
///
/// The result must contain entries for all statuses (`TODO`, `IN_PROGRESS`, `DONE`) even when the
/// total count is zero. Each entry keeps per-priority counters within a `BTreeMap`.
pub fn summarize_by_status(tasks: &[Task]) -> Vec<StatusSummary> {
    let all_statuses = Status::all();
    let mut summaries = Vec::new();

    for status in all_statuses {
        let mut counts = BTreeMap::new();
        counts.insert(Priority::High, 0);
        counts.insert(Priority::Medium, 0);
        counts.insert(Priority::Low, 0);

        for task in tasks.iter() {
            if task.status == status {
                *counts.get_mut(&task.priority).unwrap() += 1;
            }
        }

        summaries.push(StatusSummary { status, counts });
    }

    summaries
}

/// Formats aggregated data ready to be displayed.
///
/// Every line should follow the pattern
/// `TODO: 2 (high: 1, medium: 1, low: 0)`
/// while keeping priority order `high`, `medium`, `low`.
pub fn format_summary(summary: &[StatusSummary]) -> Vec<String> {
    let mut summ: Vec<String> = Vec::new();
    for i in summary {
        let status = i.status.label();
        let mut parts = Vec::new();

        for priority in Priority::all() {
            let count = i.counts.get(&priority).unwrap();
            parts.push(format!("{}: {}", priority.label(), count));
        }

        let summary = parts.join(", ");
        summ.push(format!("{}: {} ({})", status, i.total(), summary));
    }
    summ
}

/// Full pipeline: read, parse, aggregate and format.
pub fn run_from_reader<R: BufRead>(reader: R) -> Result<Vec<String>, TaskParseError> {
    let tasks = parse_tasks(&std::io::read_to_string(reader).unwrap())?;
    let summaries = summarize_by_status(&tasks);
    Ok(format_summary(&summaries))
}

/// Convenience API for tests that accepts input as a single string.
pub fn run_from_str(input: &str) -> Result<Vec<String>, TaskParseError> {
    let cursor = std::io::Cursor::new(input.as_bytes());
    run_from_reader(cursor)
}
