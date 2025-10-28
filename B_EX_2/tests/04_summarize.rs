use b_ex_2::{summarize_by_status};
use b_ex_2::domain::{Priority, Status};
use b_ex_2::domain::Priority;
fn expect_counts(summary: &b_ex_2::StatusSummary, high: usize, medium: usize, low: usize) {
    let get = |p: Priority| *summary.counts.get(&p).unwrap_or(&0);
    assert_eq!(get(Priority::High), high, "unexpected high count for {:?}", summary.status);
    assert_eq!(get(Priority::Medium), medium, "unexpected medium count for {:?}", summary.status);
    assert_eq!(get(Priority::Low), low, "unexpected low count for {:?}", summary.status);
}


#[test]
fn summarize_counts_and_total_per_status() {
    let tasks = vec![
        b_ex_2::domain::Task { title: "A".into(), priority: Priority::High, status: Status::Todo },
        b_ex_2::domain::Task { title: "B".into(), priority: Priority::Medium, status: Status::Done },
        b_ex_2::domain::Task { title: "C".into(), priority: Priority::Low, status: Status::InProgress },
        b_ex_2::domain::Task { title: "D".into(), priority: Priority::High, status: Status::Done },
    ];

    let summary = summarize_by_status(&tasks);
    assert_eq!(summary.len(), Status::all().len());

    // TODO
    expect_counts(&summary[0], 1, 0, 0);
    assert_eq!(summary[0].total(), 1);

    // IN_PROGRESS
    expect_counts(&summary[1], 0, 0, 1);
    assert_eq!(summary[1].total(), 1);

    // DONE
    expect_counts(&summary[2], 1, 1, 0);
    assert_eq!(summary[2].total(), 2);
}

#[test]
fn summarize_includes_zero_statuses_in_order() {
    let tasks = vec![];
    let summary = summarize_by_status(&tasks);
    assert_eq!(summary.len(), 3);
    assert_eq!(summary[0].status, Status::Todo);
    assert_eq!(summary[1].status, Status::InProgress);
    assert_eq!(summary[2].status, Status::Done);
    // Wszystkie zera
    for s in &summary { assert_eq!(s.total(), 0); }
}
