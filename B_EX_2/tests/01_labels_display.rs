#![allow(clippy::bool_assert_comparison)]
use b_ex_2::domain::{Priority, Status};

#[test]
fn priority_labels_and_display() {
    let expected = [("high", Priority::High), ("medium", Priority::Medium), ("low", Priority::Low)];
    for (label, p) in expected { 
        assert_eq!(p.label(), label);
        assert_eq!(p.to_string(), label);
    }
    // Kolejność raportowania
    assert_eq!(Priority::all(), [Priority::High, Priority::Medium, Priority::Low]);
}

#[test]
fn status_labels_and_display() {
    let expected = [("TODO", Status::Todo), ("IN_PROGRESS", Status::InProgress), ("DONE", Status::Done)];
    for (label, s) in expected { 
        assert_eq!(s.label(), label);
        assert_eq!(s.to_string(), label);
    }
    // Kolejność biznesowa
    assert_eq!(Status::all(), [Status::Todo, Status::InProgress, Status::Done]);
}
