use std::collections::BTreeMap;
use std::io::BufRead;

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

    impl FromStr for Priority {
        type Err = TaskParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let trimmed = s.trim();
            match trimmed.to_ascii_lowercase().as_str() {
                "high" => Ok(Priority::High),
                "medium" => Ok(Priority::Medium),
                "low" => Ok(Priority::Low),
                _ => Err(TaskParseError::InvalidPriority(trimmed.to_string())),
            }
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
            match self {Status::Todo => "TODO",
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

    impl FromStr for Status {
        type Err = TaskParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let trimmed = s.trim();
            match trimmed.to_ascii_lowercase().as_str() {
                "todo" => Ok(Status::Todo),
                "in_progress" => Ok(Status::InProgress),
                "done" => Ok(Status::Done),
                _ => Err(TaskParseError::InvalidStatus(trimmed.to_string())),
            }
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
                TaskParseError::InvalidFormat(line) => write!(f, "Niepoprawny format linii: {}", line),
                TaskParseError::InvalidPriority(value) => write!(f, "Nieznany priorytet: {}", value),
                TaskParseError::InvalidStatus(value) => write!(f, "Nieznany status: {}", value),
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
            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() != 3 {
                return Err(TaskParseError::InvalidFormat(line.to_string()));
            }

            let title = parts[0].to_string();
            let priority = parts[1].parse::<Priority>()?;
            let status = parts[2].parse::<Status>()?;
            Ok(Task { title, priority, status })
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
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse::<Task>())
        .collect();
    let tasks = tasks?;

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
    let mut summaries = Vec::new();
    for status in Status::all(){
        let mut counts = BTreeMap::new();
        for priority in Priority::all(){
            counts.insert(priority, 0);
        }
        for task in tasks {
            if task.status == status {
                *counts.entry(task.priority).or_insert(0) += 1;
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
    summary.iter().map(|s| {
        let total = s.total();
        let priority_parts: Vec<String> = Priority::all()
                .iter()
                .map(|p| {
                    let count = s.counts.get(p).unwrap_or(&0);
                    format!("{}: {}", p.label(), count)
                })
                .collect();
        format!("{}: {} ({})", s.status.label(), total, priority_parts.join(", "))
    }).collect()
}

/// Full pipeline: read, parse, aggregate and format.
pub fn run_from_reader<R: BufRead>(reader: R) -> Result<Vec<String>, TaskParseError> {
    let mut buffer = String::new();
    let mut reader = reader;
    reader.read_to_string(&mut buffer).map_err(|_| TaskParseError::NoTasks)?;

    let tasks = parse_tasks(&buffer)?;
    let summary = summarize_by_status(&tasks);
    Ok(format_summary(&summary))
}

/// Convenience API for tests that accepts input as a single string.
pub fn run_from_str(input: &str) -> Result<Vec<String>, TaskParseError> {
    let cursor = std::io::Cursor::new(input.as_bytes());
    run_from_reader(cursor)
}
