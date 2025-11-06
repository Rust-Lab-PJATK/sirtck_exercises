use std::io;

/// Zbiera tematy z pojedynczej linii wejścia rozdzielonej przecinkami.
/// Zwraca błąd `"Brak tematów do przećwiczenia."`, jeśli po oczyszczeniu nie ma żadnych wpisów.
pub fn collect_topics(line: &str) -> Result<Vec<String>, String> {
    todo!()
}

/// Buduje numerowaną checklistę w formacie `"{}. [ ] {}"` dla każdego tematu.
pub fn format_checklist(items: &[String]) -> Vec<String> {
    todo!()
}

/// Tworzy linię podsumowania z liczbą elementów i liczbą znaków bez spacji.
pub fn build_summary(items: &[String]) -> String {
    todo!()
}

/// Orkiestruje cały raport: zbiera tematy, buduje checklistę i dodaje linię podsumowania.
pub fn generate_report(line: &str) -> Result<Vec<String>, String> {
    todo!()
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
