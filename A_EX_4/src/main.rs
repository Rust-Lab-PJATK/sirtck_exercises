use std::io;

/// Zbiera tematy z pojedynczej linii wejścia rozdzielonej przecinkami.
pub fn collect_topics(line: &str) -> Result<Vec<String>, String> {
    let topics: Vec<String> = line
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    if topics.is_empty() {
        Err("Brak tematów do przećwiczenia.".to_string())
    } else {
        Ok(topics)
    }
}

/// Buduje numerowaną checklistę w formacie `"{}. [ ] {}"` dla każdego tematu.
pub fn format_checklist(items: &[String]) -> Vec<String> {
    items
        .iter()
        .enumerate()
        .map(|(i, item)| format!("{}. [ ] {}", i + 1, item))
        .collect()
}

/// Tworzy linię podsumowania z liczbą elementów i liczbą znaków bez spacji.
pub fn build_summary(items: &[String]) -> String {
    let count = items.len();

    let char_count: usize = items
        .iter()
        .map(|topic| topic.chars().filter(|c| !c.is_whitespace()).count())
        .sum();

    format!(
        "Podsumowanie: {} elementów, {} znaków (bez spacji)",
        count, char_count
    )
}

/// Orkiestruje cały raport.
pub fn generate_report(line: &str) -> Result<Vec<String>, String> {
    let topics = collect_topics(line)?;
    let mut result = format_checklist(&topics);

    let summary = build_summary(&topics);
    result.push(summary);

    Ok(result)
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