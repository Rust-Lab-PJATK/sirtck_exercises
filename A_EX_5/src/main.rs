use std::io;

/// Parsuje liczby całkowite rozdzielone białymi znakami.
/// Zwraca komunikat `"Brak liczb do przeanalizowania."`, jeśli po przetworzeniu nie ma żadnych wartości.
pub fn parse_numbers(input: &str) -> Result<Vec<i32>, String> {
    todo!()
}

/// Zwraca krotkę (liczba elementów, minimum, maksimum, suma) dla przekazanych liczb.
pub fn summarize_numbers(numbers: &[i32]) -> (usize, i32, i32, i32) {
    todo!()
}

/// Buduje cztery linie raportu na podstawie przekazanych liczb.
pub fn describe_numbers(numbers: &[i32]) -> Vec<String> {
    todo!()
}

/// Odpowiada za pełną analizę: parsowanie wejścia i przygotowanie raportu.
pub fn run_analysis(line: &str) -> Result<Vec<String>, String> {
    todo!()
}

pub fn main() {
    let mut buffer = String::new();

    if io::stdin().read_line(&mut buffer).is_err() {
        println!("Nie udało się odczytać danych.");
        return;
    }

    match run_analysis(buffer.as_str()) {
        Ok(report) => {
            for line in report {
                println!("{line}");
            }
        }
        Err(message) => println!("{message}"),
    }
}
