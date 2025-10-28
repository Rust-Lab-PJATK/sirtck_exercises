use b_ex_2::{run_from_reader};

#[test]
fn run_from_reader_happy_path() {
    let input = "\
        Ship layout | high | todo\n        Update docs | medium | done\n        Pairing session | low | in_progress\n        Refactor module | high | done\n    ";
    let cursor = std::io::Cursor::new(input.as_bytes());
    let lines = run_from_reader(cursor).expect("input should parse");
    assert_eq!(lines, vec![
        "TODO: 1 (high: 1, medium: 0, low: 0)",
        "IN_PROGRESS: 1 (high: 0, medium: 0, low: 1)",
        "DONE: 2 (high: 1, medium: 1, low: 0)"
    ]);
}

#[test]
fn run_from_reader_bubbles_up_errors() {
    let cursor = std::io::Cursor::new("x | urgent | todo".as_bytes());
    let err = run_from_reader(cursor).expect_err("should fail on invalid priority");
    assert_eq!(err.to_string(), "Nieznany priorytet: urgent");
}
