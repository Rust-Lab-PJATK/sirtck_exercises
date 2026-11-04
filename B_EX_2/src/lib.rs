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
                Priority::Low => "low"
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
                Status::Done => "DONE"
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
                TaskParseError::InvalidFormat(input) => write!(f, "Niepoprawny format linii: {}", input),
                TaskParseError::InvalidPriority(input) => write!(f, "Nieznany priorytet: {}", input),
                TaskParseError::InvalidStatus(input) => write!(f, "Nieznany status: {}", input)
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
            let parts: Vec<&str> = line.split('|').map(str::trim).collect();

            let [title, priority, status] = parts[..] else {
                return Err(TaskParseError::InvalidFormat(line.to_string()));
            };
            Ok(Task {
                title: {title.to_string()},
                priority: {
                    match priority.to_lowercase().as_str() {
                        "high" => Priority::High,
                        "medium" => Priority::Medium,
                        "low" => Priority::Low,
                        _ => return Err(TaskParseError::InvalidPriority(priority.to_string()))
                    }
                },
                status: {
                    match status.to_lowercase().as_str() {
                        "todo" => Status::Todo,
                        "in_progress" => Status::InProgress,
                        "done" => Status::Done,
                        _ => return Err(TaskParseError::InvalidStatus(status.to_string()))
                    }
                }
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
        self.counts.values().fold(0, |acc, &x| acc + x)
    }
}

/// Parses multiple tasks from raw text, skipping empty lines.
///
/// At least one valid task must be present or [`TaskParseError::NoTasks`] is returned.
pub fn parse_tasks(input: &str) -> Result<Vec<Task>, TaskParseError> {
    let mut tasks: Vec<Task> = vec![];
    for mut line in input.lines() {
        line = line.trim();
        if !line.is_empty() {
            tasks.push(line.parse()?)
        }
    }
    match tasks.len() {
        0 => Result::Err(TaskParseError::NoTasks),
        _ => Result::Ok(tasks)
    }
}

/// Builds summaries for every status in the required order.
///
/// The result must contain entries for all statuses (`TODO`, `IN_PROGRESS`, `DONE`) even when the
/// total count is zero. Each entry keeps per-priority counters within a `BTreeMap`.
pub fn summarize_by_status(tasks: &[Task]) -> Vec<StatusSummary> {
    let mut statuses: BTreeMap<Status,StatusSummary> = BTreeMap::new();
    for status in Status::all() {
        let mut priorities = BTreeMap::new();
        for priority in Priority::all() {
            priorities.insert(priority, 0);
        }
        statuses.insert(status, StatusSummary {status, counts: priorities});
    }

    for task in tasks {
        *statuses.get_mut(&task.status).unwrap().counts.entry(task.priority).or_insert(0) += 1;
    }
    statuses.into_values().collect()
}

/// Formats aggregated data ready to be displayed.
///
/// Every line should follow the pattern
/// `TODO: 2 (high: 1, medium: 1, low: 0)`
/// while keeping priority order `high`, `medium`, `low`.
pub fn format_summary(summary: &[StatusSummary]) -> Vec<String> {
    let mut result = vec![];
    for entry in summary {
        let count = entry.counts.values().fold(0, |acc, &x| acc + x);
        result.push(format!("{}: {} (high: {}, medium: {}, low: {})", entry.status, count, entry.counts.get(&Priority::High).unwrap(), entry.counts.get(&Priority::Medium).unwrap(), entry.counts.get(&Priority::Low).unwrap()))
    }
    result
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