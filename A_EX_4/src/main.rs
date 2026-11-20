use std::io;

/// Zbiera tematy z pojedynczej linii wejścia rozdzielonej przecinkami.
/// Zwraca błąd `"Brak tematów do przećwiczenia."`, jeśli po oczyszczeniu nie ma żadnych wpisów.
pub fn collect_topics(line: &str) -> Result<Vec<String>, String> {
    let res: Vec<String> = line.split(',').map(|str| str.trim().to_string()).collect();
    if res.is_empty() {
        Err(String::from("Brak tematów do przećwiczenia."))
    } else {
        Ok(res)
    }
}

/// Buduje numerowaną checklistę w formacie `"{}. [ ] {}"` dla każdego tematu.
pub fn format_checklist(items: &[String]) -> Vec<String> {
    items
        .iter()
        .enumerate()
        .map(|(i, str)| format!("{}. [ ] {str}", i + 1))
        .collect()
}

/// Tworzy linię podsumowania z liczbą elementów i liczbą znaków bez spacji.
pub fn build_summary(items: &[String]) -> String {
    let sum = items.iter().count();
    let sum_no_space: usize = items
        .iter()
        .map(|str| str.chars().filter(|chr| !chr.is_whitespace()).count())
        .sum();
    format!("Podsumowanie: {sum} elementow, znaków {sum_no_space} (bez spacji)")
}

/// Orkiestruje cały raport: zbiera tematy, buduje checklistę i dodaje linię podsumowania.
pub fn generate_report(line: &str) -> Result<Vec<String>, String> {
    match collect_topics(line) {
        Ok(topics) => {
            let mut checklist = format_checklist(&topics);
            checklist.push(build_summary(&checklist));
            Ok(checklist)
        }
        Err(err) => Err(err),
    }
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
