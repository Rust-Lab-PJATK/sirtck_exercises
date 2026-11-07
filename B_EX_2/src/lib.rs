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
                TaskParseError::InvalidFormat(line) => write!(f, "Niepoprawny format linii: {}", line),
                TaskParseError::InvalidPriority(priority) => write!(f, "Nieznany priorytet: {}", priority),
                TaskParseError::InvalidStatus(status) => write!(f, "Nieznany status: {}", status)
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
            let words: Vec<&str> = line.split("|").collect();
            if words.len() != 3 {
                return Err(TaskParseError::InvalidFormat(String::from(line)));
            }
            let title = words[0].trim();
            let priority_string = words[1].trim();
            let priority = match priority_string.to_ascii_lowercase().as_str() {
                "high" => Priority::High,
                "medium" => Priority::Medium,
                "low" => Priority::Low,
                _ => return Err(TaskParseError::InvalidPriority(String::from(priority_string)))
            };

            let status_string = words[2].trim();
            let status = match status_string.to_ascii_uppercase().as_str() {
                "TODO" => Status::Todo,
                "IN_PROGRESS" => Status::InProgress,
                "DONE" => Status::Done,
                _ => return Err(TaskParseError::InvalidStatus(String::from(status_string)))
            };
            let title = String::from(title);
            return Ok(Task { title, priority, status });
            
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
        let mut total = 0;
        for count in self.counts.values() {
            total += count;
        }
        return total;
    }
}

/// Parses multiple tasks from raw text, skipping empty lines.
///
/// At least one valid task must be present or [`TaskParseError::NoTasks`] is returned.
pub fn parse_tasks(input: &str) -> Result<Vec<Task>, TaskParseError> {
    use std::str::FromStr;
    let mut tasks = Vec::<Task>::new();
    let mut task_found = false;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let task = Task::from_str(line)?;
        tasks.push(task);
        task_found = true;
    }
    if !task_found {
        return Err(TaskParseError::NoTasks);
    }
    return Ok(tasks);
}

/// Builds summaries for every status in the required order.
///
/// The result must contain entries for all statuses (`TODO`, `IN_PROGRESS`, `DONE`) even when the
/// total count is zero. Each entry keeps per-priority counters within a `BTreeMap`.
pub fn summarize_by_status(tasks: &[Task]) -> Vec<StatusSummary> {
    let mut aggregation = BTreeMap::<Status, BTreeMap<Priority, usize>>::new();
    for status in Status::all() {
        let mut count_map = BTreeMap::<Priority, usize>::new();
        for priority in Priority::all() {
            count_map.insert(priority, 0);
        }
        aggregation.insert(status, count_map);
    }
    for task in tasks {
        *aggregation
            .get_mut(&task.status)
            .unwrap()
            .get_mut(&task.priority)
            .unwrap() += 1;
    }

    let mut summaries = Vec::<StatusSummary>::new();
    for (status, counts) in aggregation {
        summaries.push(StatusSummary { status, counts });
    }
    return summaries;
}

/// Formats aggregated data ready to be displayed.
///
/// Every line should follow the pattern
/// `TODO: 2 (high: 1, medium: 1, low: 0)`
/// while keeping priority order `high`, `medium`, `low`.
pub fn format_summary(summary: &[StatusSummary]) -> Vec<String> {
    let mut output = Vec::<String>::new();
    for summary in summary {
        let status = format!("{}: {}", summary.status.label(), summary.total());
        let mut priorities = Vec::<String>::new();
        for (priority, count) in summary.counts.iter() {
            priorities.push(format!("{}: {}", priority.label(), count));
        }
        let priorities = priorities.join(", ");
        output.push(format!("{} ({})", status, priorities));
    }
    return output;
}

/// Full pipeline: read, parse, aggregate and format.
pub fn run_from_reader<R: BufRead>(reader: R) -> Result<Vec<String>, TaskParseError> {
    let mut reader = reader;
    let mut input = String::new();
    reader.read_to_string(&mut input).map_err(|_| TaskParseError::NoTasks)?;
    let tasks = parse_tasks(&input)?;
    let summaries = summarize_by_status(&tasks);
    let formatted = format_summary(&summaries);
    return Ok(formatted);
}

/// Convenience API for tests that accepts input as a single string.
pub fn run_from_str(input: &str) -> Result<Vec<String>, TaskParseError> {
    let cursor = std::io::Cursor::new(input.as_bytes());
    run_from_reader(cursor)
}
