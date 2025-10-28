use b_ex_2::{parse_tasks};
use b_ex_2::domain::{Status, TaskParseError};

#[test]
fn parse_tasks_skips_blank_lines_and_collects_tasks() {
    let input = "\n  A | high | todo  \n\n B | medium | done\n\t\nC|low|in_progress \n";
    let tasks = parse_tasks(input).expect("should parse tasks");
    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].title, "A");
    assert_eq!(tasks[1].title, "B");
    assert_eq!(tasks[2].title, "C");
}

#[test]
fn parse_tasks_requires_at_least_one_non_empty_line() {
    let err = parse_tasks("   \n\t ").expect_err("blank input should fail");
    assert!(matches!(err, TaskParseError::NoTasks));
}

#[test]
fn parse_tasks_stops_on_first_invalid_line() {
    // Pierwsza linia poprawna, druga ma zły status – powinniśmy dostać błąd (a nie ignorować linii).
    let input = "ok | high | todo\nboom | medium | paused\nignored | low | done";
    let err = parse_tasks(input).expect_err("invalid line should error out");
    assert_eq!(err.to_string(), "Nieznany status: paused");
}
