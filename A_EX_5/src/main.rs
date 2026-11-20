use std::io;

/// Parsuje liczby całkowite rozdzielone białymi znakami.
/// Zwraca komunikat `"Brak liczb do przeanalizowania."`, jeśli po przetworzeniu nie ma żadnych wartości.
pub fn parse_numbers(input: &str) -> Result<Vec<i32>, String> {
    let a = input.split_whitespace();
    if a.clone().count() > 0 {
        Ok(a.map(|x| x.parse::<i32>().unwrap()).collect())
    } else {
        Err(String::from("Brak liczb do przeanalizowania."))
    }
}

/// Zwraca krotkę (liczba elementów, minimum, maksimum, suma) dla przekazanych liczb.
pub fn summarize_numbers(numbers: &[i32]) -> (usize, i32, i32, i32) {
    let elements: usize = numbers.len();
    let minimum: i32 = *numbers.iter().min().unwrap();
    let maximum: i32 = *numbers.iter().max().unwrap();
    let sum: i32 = numbers.iter().sum();
    (elements, minimum, maximum, sum)
}

/// Buduje cztery linie raportu na podstawie przekazanych liczb.
pub fn describe_numbers(numbers: &[i32]) -> Vec<String> {
    let summ = summarize_numbers(numbers);
    vec![
        format!(
            "Liczby ({}): {:?}",
            summ.0,
            numbers
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        ),
        format!("Minimum: {}", summ.1),
        format!("Maximum: {}", summ.2),
        format!("Suma: {}", summ.3),
    ]
}

/// Odpowiada za pełną analizę: parsowanie wejścia i przygotowanie raportu.
pub fn run_analysis(line: &str) -> Result<Vec<String>, String> {
    let nums = parse_numbers(line);
    match nums {
        Ok(nums) => Ok(describe_numbers(&nums)),
        Err(nums) => Err(nums),
    }
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
