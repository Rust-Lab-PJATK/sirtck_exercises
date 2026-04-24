use std::io;

/// Zbiera tematy z pojedynczej linii wejścia rozdzielonej przecinkami.
/// Zwraca błąd `"Brak tematów do przećwiczenia."`, jeśli po oczyszczeniu nie ma żadnych wpisów.
pub fn collect_topics(line: &str) -> Result<Vec<String>, String> {
    ///todo!()
    ///

    let cleaned: Vec<String> = line
        .split(',')
        .map(|chunk| chunk.trim())
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| chunk.to_string())
        .collect();

    if cleaned.is_empty() {
        Err("Brak tematów do przećwiczenia".to_string())
    } else {
        Ok(cleaned)
    }
}

/// Buduje numerowaną checklistę w formacie `"{}. [ ] {}"` dla każdego tematu.
pub fn format_checklist(items: &[String]) -> Vec<String> {

    ///todo!()
    ///
    let checklist = items.iter().enumerate().map(|(index,value)|format!("{}. [ ] {value}", index + 1)).collect();

    checklist
}

/// Tworzy linię podsumowania z liczbą elementów i liczbą znaków bez spacji.
pub fn build_summary(items: &[String]) -> String {
    ///todo!()
    let count = items.len();
    let char_count: usize = items.iter().map(|item| item.chars().filter(|c| !c.is_whitespace()).count()).sum();


    format!("Podsumowanie: {} elementów, {} znaków (bez spacji)", count, char_count)


}

/// Orkiestruje cały raport: zbiera tematy, buduje checklistę i dodaje linię podsumowania.
pub fn generate_report(line: &str) -> Result<Vec<String>, String> {
    let topics = collect_topics(line)?;
    let mut checklist = format_checklist(&topics);
    let summary = build_summary(&topics);
    checklist.push(summary);

    Ok(checklist)
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
