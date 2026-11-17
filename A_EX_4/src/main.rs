use std::io;

/// Zbiera tematy z pojedynczej linii wejścia rozdzielonej przecinkami.
/// Zwraca błąd `"Brak tematów do przećwiczenia."`, jeśli po oczyszczeniu nie ma żadnych wpisów.
pub fn collect_topics(line: &str) -> Result<Vec<String>, String> {
    //todo!()
    let cleaned: Vec<String> = line
    .split(',')
    .map(|chunk| chunk.trim())
    .filter(|chunk| !chunk.is_empty())
    .map(|chunk| chunk.to_string())
    .collect();

    if cleaned.is_empty() {
        return Err("Brak tematów do przećwiczenia.".to_string());
    }
    Ok(cleaned)
}

/// Buduje numerowaną checklistę w formacie `"{}. [ ] {}"` dla każdego tematu.
pub fn format_checklist(items: &[String]) -> Vec<String> {
    //todo!()
    let checklist: Vec<String> = items
    .iter()
    .enumerate()
    .map(|(index, value)| format!("{}. [ ] {}", index + 1, value))
    .collect();

    checklist
}



/// Orkiestruje cały raport: zbiera tematy, buduje checklistę i dodaje linię podsumowania.
pub fn build_summary(items: &[String]) -> String {
    let count = items.len();

    let char_count: usize = items
        .iter()
        .map(|topic| topic.chars().filter(|c| !c.is_whitespace()).count())
        .sum();

    format!(
        "Podsumowanie: {} elementów, {} znaków (bez spacji)",
        count,
        char_count
    )
}

/// Orkiestruje cały raport: zbiera tematy, buduje checklistę i dodaje linię podsumowania.
pub fn generate_report(line: &str) -> Result<Vec<String>, String> {
    let topics = collect_topics(line)?;

    let mut report = format_checklist(&topics);

    report.push(build_summary(&topics));

    Ok(report)
}

pub fn main() {
    let mut buffer = String::new();

    if io::stdin().read_line(&mut buffer).is_err() {
        println!("Nie udało się odczytać danych.");
        return;
    }

    match generate_report(buffer.as_str()) {
        Ok(lines) => {
            for line in lines {
                println!("{line}");
            }
        }
        Err(message) => println!("{message}"),
    }
}
