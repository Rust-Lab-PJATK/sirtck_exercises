use std::io;

/// Zbiera tematy z pojedynczej linii wejścia rozdzielonej przecinkami.
/// Zwraca błąd `"Brak tematów do przećwiczenia."`, jeśli po oczyszczeniu nie ma żadnych wpisów.
pub fn collect_topics(line: &str) -> Result<Vec<String>, String> {
    let tematy: Vec<String> = line.split(",")
        .map(|w| w.trim())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_string())
        .collect();
    if tematy.is_empty() { return Err(String::from("empty line")) }
    Ok(tematy)
}

/// Buduje numerowaną checklistę w formacie `"{}. [ ] {}"` dla każdego tematu.
pub fn format_checklist(items: &[String]) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for (index, item) in items.iter().enumerate() {
        output.push(format!("{index}. [ ] {item}"))
    }
    output
}

/// Tworzy linię podsumowania z liczbą elementów i liczbą znaków bez spacji.
pub fn build_summary(items: &[String]) -> String {
    let topic_count: usize = items.len();
    let mut topic_char_len: usize = 0;
    for item in items {
        topic_char_len += item.chars().filter(|c| !c.is_whitespace()).count();
    }
    format!("Podsumowanie: {topic_count} elementów, {topic_char_len} znaków (bez spacji)")
}

/// Orkiestruje cały raport: zbiera tematy, buduje checklistę i dodaje linię podsumowania.
pub fn generate_report(line: &str) -> Result<Vec<String>, String> {
    let mut output: Vec<String> = Vec::new();
    let topics = collect_topics(line)?;
    output.extend_from_slice(&format_checklist(&topics));
    output.push(build_summary(&topics));
    Ok(output)
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
