use std::io;

/// Interpretuje jedną linię wejścia.
/// - `Ok(Some(value))` oznacza liczbę całkowitą, którą trzeba dodać do sumy.
/// - `Ok(None)` oznacza komendę zakończenia (`koniec`).
/// - `Err(message)` zawiera komunikat, który należy wypisać na stdout.
pub fn parse_line(line: &str) -> Result<Option<i32>, String> {
    ///todo!()
    let trimmed = line.trim();


    if trimmed == "koniec" {
        return Ok(None);
    }

    match trimmed.parse::<i32>()
    {
        Ok(value) => Ok(Some(value)),
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
    ///todo!()
    let mut count = 0;
    let mut messages = Vec::new();

    for line in lines{


        let text = line.as_ref();

        match parse_line(text) {
            Ok(Some(value)) => {
                count += value;
                messages.push(format!("Aktualna suma: {}", count));
            }
            Ok(None) => {
                messages.push(format!("Zamykam program. Suma: {}", count));
                break;
            }
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
