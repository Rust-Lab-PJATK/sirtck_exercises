use b_ex_2::domain::TaskParseError;

#[test]
fn error_messages_match_spec() {
    let err = TaskParseError::InvalidFormat("broken".to_string());
    assert_eq!(err.to_string(), "Niepoprawny format linii: broken");

    let err = TaskParseError::InvalidPriority("Urgent".to_string());
    assert_eq!(err.to_string(), "Nieznany priorytet: Urgent");

    let err = TaskParseError::InvalidStatus("Paused".to_string());
    assert_eq!(err.to_string(), "Nieznany status: Paused");

    let err = TaskParseError::NoTasks;
    assert_eq!(err.to_string(), "Brak zadan");
}
