use std::io;

/// Interpretuje jedną linię wejścia.
pub fn parse_line(line: &str) -> Result<Option<i32>, String> {
    let trimmed = line.trim();

    // pusta linia
    if trimmed.is_empty() {
        return Err("Wpisz liczbę lub 'koniec'.".to_string());
    }

    // komenda zakończenia
    if trimmed == "koniec" {
        return Ok(None);
    }

    // próba parsowania liczby
    match trimmed.parse::<i32>() {
        Ok(value) => Ok(Some(value)),
        Err(_) => Err(format!("Niepoprawna liczba: {}", trimmed)),
    }
}

/// Przetwarza sekwencję linii tekstu i zwraca komunikaty do wypisania przez program.
pub fn run_session<I>(lines: I) -> Vec<String>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut total = 0;
    let mut output = Vec::new();

    for line in lines {
        match parse_line(line.as_ref()) {
            Ok(Some(value)) => {
                total += value;
                output.push(format!("Aktualna suma: {}", total));
            }
            Ok(None) => {
                output.push(format!("Zamykam program. Suma: {}", total));
                break;
            }
            Err(message) => {
                output.push(message);
            }
        }
    }

    output
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