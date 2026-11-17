use std::io;

/// Interpretuje jedną linię wejścia.
/// - `Ok(Some(value))` oznacza liczbę całkowitą, którą trzeba dodać do sumy.
/// - `Ok(None)` oznacza komendę zakończenia (`koniec`).
/// - `Err(message)` zawiera komunikat, który należy wypisać na stdout.
pub fn parse_line(line: &str) -> Result<Option<i32>, String> {
    let trimmed = line.trim();

    if trimmed.is_empty() {
        return Err("Wpisz liczbę lub 'koniec'.".to_string());
    }

    if trimmed == "koniec" {
        return Ok(None);
    }

    match trimmed.parse::<i32>() {
        Ok(number) => Ok(Some(number)),
        Err(_) => Err(format!("Niepoprawna liczba: {}", trimmed)),
    }
}

/// Przetwarza sekwencję linii tekstu i zwraca komunikaty do wypisania przez program.
/// Powinna używać `parse_line`, aktualizować sumę liczb i zakończyć działanie po `Ok(None)`.
pub fn run_session<I>(lines: I) -> Vec<String>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    //todo!()
    let mut total = 0;
    let mut messages = Vec::new();

    for line in lines {
        match parse_line(line.as_ref()) {
            Ok(Some(value)) => {
                total += value;
                messages.push(format!("Aktualna suma: {}", total));
            }
            Ok(None) => {
                messages.push(format!("Zamykam program. Suma: {}", total));
                break;
            }
            Err(message) => {
                messages.push(message);
            }
        }
    }

    messages
}

pub fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut total = 0;

    loop {
        buffer.clear();

        if stdin.read_line(&mut buffer).is_err() {
            println!("Wpisz liczbę lub 'koniec'.");
            continue;
        }

        match parse_line(buffer.as_str()) {
            Ok(Some(value)) => {
                total += value;
                println!("Aktualna suma: {}", total);
            }
            Ok(None) => {
                println!("Zamykam program. Suma: {}", total);
                break;
            }
            Err(message) => {
                println!("{}", message);
            }
        }
    }
}
