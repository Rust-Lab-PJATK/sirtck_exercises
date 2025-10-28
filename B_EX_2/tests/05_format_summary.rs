use b_ex_2::{format_summary, summarize_by_status};
use b_ex_2::domain::{Priority, Status};

#[test]
fn format_summary_matches_expected_lines_and_order() {
    let tasks = vec![
        b_ex_2::domain::Task { title: "Ship".into(), priority: Priority::High, status: Status::Todo },
        b_ex_2::domain::Task { title: "Docs".into(), priority: Priority::Medium, status: Status::Done },
        b_ex_2::domain::Task { title: "Pair".into(), priority: Priority::Low, status: Status::InProgress },
        b_ex_2::domain::Task { title: "Refactor".into(), priority: Priority::High, status: Status::Done },
    ];
    let summary = summarize_by_status(&tasks);
    let lines = format_summary(&summary);

    assert_eq!(lines, vec![
        "TODO: 1 (high: 1, medium: 0, low: 0)",
        "IN_PROGRESS: 1 (high: 0, medium: 0, low: 1)",
        "DONE: 2 (high: 1, medium: 1, low: 0)"
    ]);
}

#[test]
fn format_summary_includes_zeros_for_missing_priorities() {
    let tasks = vec![
        b_ex_2::domain::Task { title: "Only".into(), priority: Priority::Medium, status: Status::Todo },
    ];
    let summary = summarize_by_status(&tasks);
    let lines = format_summary(&summary);

    assert_eq!(lines[0], "TODO: 1 (high: 0, medium: 1, low: 0)");
    assert_eq!(lines[1], "IN_PROGRESS: 0 (high: 0, medium: 0, low: 0)");
    assert_eq!(lines[2], "DONE: 0 (high: 0, medium: 0, low: 0)");
}
