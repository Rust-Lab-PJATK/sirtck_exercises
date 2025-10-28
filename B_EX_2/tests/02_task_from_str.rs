use std::str::FromStr;
use b_ex_2::domain::{Task, Priority, Status, TaskParseError};

#[test]
fn task_from_str_trims_and_is_case_insensitive() {
    let t = Task::from_str("  Ship layout   |   High   |  in_progress  ").expect("should parse");
    assert_eq!(t.title, "Ship layout");
    assert_eq!(t.priority, Priority::High);
    assert_eq!(t.status, Status::InProgress);

    let t = Task::from_str("Refactor | low | DONE").expect("should also parse");
    assert_eq!(t.title, "Refactor");
    assert_eq!(t.priority, Priority::Low);
    assert_eq!(t.status, Status::Done);
}

#[test]
fn task_from_str_invalid_format_too_few_or_too_many_fields() {
    let err = Task::from_str("missing separators").expect_err("format must be invalid");
    assert!(matches!(err, TaskParseError::InvalidFormat(raw) if raw == "missing separators"));

    let err = Task::from_str("a | b | c | d").expect_err("too many fields must be invalid");
    assert!(matches!(err, TaskParseError::InvalidFormat(raw) if raw == "a | b | c | d"));
}

#[test]
fn task_from_str_unknown_priority_and_status_preserve_raw_token() {
    // Priorytet – zachowaj oryginalne litery po przycięciu białych znaków.
    let err = Task::from_str("Demo |  UrGent  | todo").expect_err("unknown priority should fail");
    assert!(matches!(err, TaskParseError::InvalidPriority(raw) if raw == "UrGent"));

    // Status – to samo zachowanie.
    let err = Task::from_str("Demo | high | Paused ").expect_err("unknown status should fail");
    assert!(matches!(err, TaskParseError::InvalidStatus(raw) if raw == "Paused"));
}
