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
            //todo!("return the lowercase label for each priority")
            match self {
                Priority::High => "high",
                Priority::Medium => "medium",
                Priority::Low => "low",
            }
        }
    }

    impl fmt::Display for Priority {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            //todo!("print the label defined in Priority::label")
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
            //todo!("return the uppercase label for each status")
            match self {
                Status::Todo => "TODO",
                Status::InProgress => "IN_PROGRESS",
                Status::Done => "DONE",
            }
        }
    }

    impl fmt::Display for Status {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            //todo!("print the label defined in Status::label")
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
            //todo!("map error variants to human readable messages defined in README")
                         match self {
                TaskParseError::NoTasks => {
                    write!(f, "Brak zadan")
                }
                TaskParseError::InvalidFormat(line) => {
                    write!(f, "Niepoprawny format linii: {line}")
                }
                TaskParseError::InvalidPriority(value) => {
                    write!(f, "Nieznany priorytet: {value}")
                }
                TaskParseError::InvalidStatus(value) => {
                    write!(f, "Nieznany status: {value}")
                }
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
           // todo!("split the fields, trim them and return errors that highlight the issue")
            let czesci: Vec<&str> = line.split('|').collect();

            if czesci.len() != 3 {
                return Err(TaskParseError::InvalidFormat(line.to_string()));
            }

            let title = czesci[0].trim();
            let priority_raw = czesci[1].trim();
            let status_raw = czesci[2].trim();

            let priority = match priority_raw.to_ascii_lowercase().as_str() {
                "high" => Priority::High,
                "medium" => Priority::Medium,
                "low" => Priority::Low,
                _ => {
                    return Err(TaskParseError::InvalidPriority(
                        priority_raw.to_string(),
                    ))
                }
            };

            let status = match status_raw.to_ascii_lowercase().as_str() {
                "todo" => Status::Todo,
                "in_progress" => Status::InProgress,
                "done" => Status::Done,
                _ => {
                    return Err(TaskParseError::InvalidStatus(status_raw.to_string()))
                }
            };

            Ok(Task {
                title: title.to_string(),
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
        //todo!("sum all values stored in the counts map")
        self.counts.values().sum()
    }
}

/// Parses multiple tasks from raw text, skipping empty lines.
///
/// At least one valid task must be present or [`TaskParseError::NoTasks`] is returned.
pub fn parse_tasks(input: &str) -> Result<Vec<Task>, TaskParseError> {
    //todo!("split the input into lines, skip blanks and delegate to Task::from_str")
        let mut tasks = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        tasks.push(line.parse::<Task>()?);
    }

    if tasks.is_empty() {
        return Err(TaskParseError::NoTasks);
    }

    Ok(tasks)
}

/// Builds summaries for every status in the required order.
///
/// The result must contain entries for all statuses (`TODO`, `IN_PROGRESS`, `DONE`) even when the
/// total count is zero. Each entry keeps per-priority counters within a `BTreeMap`.
pub fn summarize_by_status(tasks: &[Task]) -> Vec<StatusSummary> {
    //todo!("initialise summaries for all statuses and increment counts per priority")
        let mut summaries = Vec::new();

    for status in Status::all() {
        let mut counts = BTreeMap::new();

        for priority in Priority::all() {
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
    //todo!("build the expected lines based on summarize_by_status output")
        summary
        .iter()
        .map(|item| {
            let high = item.counts.get(&Priority::High).copied().unwrap_or(0);
            let medium = item.counts.get(&Priority::Medium).copied().unwrap_or(0);
            let low = item.counts.get(&Priority::Low).copied().unwrap_or(0);

            format!(
                "{}: {} (high: {}, medium: {}, low: {})",
                item.status,
                item.total(),
                high,
                medium,
                low
            )
        })
        .collect()
}

/// Full pipeline: read, parse, aggregate and format.
pub fn run_from_reader<R: BufRead>(reader: R) -> Result<Vec<String>, TaskParseError> {
    //todo!("read the text, call parse_tasks -> summarize_by_status -> format_summary")
        let mut input = String::new();

    for line in reader.lines() {
        let line = line.map_err(|_| TaskParseError::NoTasks)?;
        input.push_str(&line);
        input.push('\n');
    }

    let tasks = parse_tasks(&input)?;
    let summary = summarize_by_status(&tasks);

    Ok(format_summary(&summary))
}

/// Convenience API for tests that accepts input as a single string.
pub fn run_from_str(input: &str) -> Result<Vec<String>, TaskParseError> {
    let cursor = std::io::Cursor::new(input.as_bytes());
    run_from_reader(cursor)
}
