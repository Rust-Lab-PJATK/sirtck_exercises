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
                Priority::Low => "low"
            }
        }
    }

    impl FromStr for Priority {
        type Err = TaskParseError;

        fn from_str(raw: &str) -> Result<Self, Self::Err> {
            match raw.trim().to_ascii_lowercase().as_str() {
                "high" => Ok(Priority::High),
                "medium" => Ok(Priority::Medium),
                "low" => Ok(Priority::Low),
                other => Err(TaskParseError::InvalidPriority(other.into())),
            }
        }
    }

    impl fmt::Display for Priority {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            Ok(println!("{}", self.label()))
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
                Status::Done => "DONE"
            }
        }
    }

    impl FromStr for Status {
        type Err = TaskParseError;

        fn from_str(raw: &str) -> Result<Self, Self::Err> {
            match raw.trim().to_ascii_lowercase().as_str() {
                "todo" => Ok(Status::Todo),
                "in_progress" => Ok(Status::InProgress),
                "done" => Ok(Status::Done),
                other => Err(TaskParseError::InvalidPriority(other.into())),
            }
        }
    }

    impl fmt::Display for Status {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            Ok(println!("{}", self.label()))
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
                TaskParseError::NoTasks => write!(f, "No tasks provided"),
                TaskParseError::InvalidFormat(value) => write!(f, "Invalid format: {}", value),
                TaskParseError::InvalidPriority(value) => write!(f, "Invalid priority: {}", value),
                TaskParseError::InvalidStatus(value) => write!(f, "Invalid status: {}", value),
                _ => write!(f, "Unknown Error"),
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
            let aspects = line.split('|').collect::<Vec<&str>>();

            for i in (3..aspects.len()).step_by(3) {

            }
            Ok(Task{title: aspects[0].trim().to_owned(), priority: Priority::from_str(aspects[1])?, status:  Status::from_str(aspects[2])?})
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
        self.counts.values().copied().sum::<usize>()
    }
}

/// Parses multiple tasks from raw text, skipping empty lines.
///
/// At least one valid task must be present or [`TaskParseError::NoTasks`] is returned.
pub fn parse_tasks(input: &str) -> Result<Vec<Task>, TaskParseError> {
    let mut lines: Vec<&str> = input.split('\n').collect();
    lines.pop();
    let mut tasks: Vec<Task> = Vec::new();

    for line in lines {
        tasks.push(Task::from_str(&line)?);
    }

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
    let mut summaries: Vec<StatusSummary> = Vec::new();
    for task in tasks {
        if let Some(existing) = summaries.iter_mut().find(|s| s.status == task.status) {
            existing.counts.insert(task.priority, 1);
        }
        else {
            let mut status_summary = StatusSummary { status: task.status, counts: BTreeMap::from([(Priority::High, 0), (Priority::Medium, 0), (Priority::Low, 0)]) };
        status_summary.counts.insert(task.priority, 1);
        summaries.push(status_summary);
        }
    }
    summaries
}

/// Formats aggregated data ready to be displayed.
///
/// Every line should follow the pattern
/// `TODO: 2 (high: 1, medium: 1, low: 0)`
/// while keeping priority order `high`, `medium`, `low`.
pub fn format_summary(summary: &[StatusSummary]) -> Vec<String> {
    let mut strings: Vec<String> = Vec::new();
    for status_summary in summary {
        let mut str = String::new();
        str = format!("{}: {} (high: {}, medium: {}, low: {})", status_summary.status.label(), status_summary.total(), status_summary.counts[&Priority::High], status_summary.counts[&Priority::Medium], status_summary.counts[&Priority::Low]);
        strings.push(str);
    }
    strings
}

/// Full pipeline: read, parse, aggregate and format.
pub fn run_from_reader<R: BufRead>(reader: R) -> Result<Vec<String>, TaskParseError> {
    let mut text = String::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => return Err(TaskParseError::InvalidFormat(err.to_string())),
        };
        text += &(line + "\n");
    }
    let tasks = parse_tasks(&text)?;
    let summary = summarize_by_status(&tasks);
    let formatted = format_summary(&summary);

    Ok(formatted)
}

/// Convenience API for tests that accepts input as a single string.
pub fn run_from_str(input: &str) -> Result<Vec<String>, TaskParseError> {
    let cursor = std::io::Cursor::new(input.as_bytes());
    run_from_reader(cursor)
}
