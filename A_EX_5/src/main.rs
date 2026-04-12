use std::io;

/// Parsuje liczby całkowite rozdzielone białymi znakami.
/// Zwraca komunikat `"Brak liczb do przeanalizowania."`, jeśli po przetworzeniu nie ma żadnych wartości.
pub fn parse_numbers(input: &str) -> Result<Vec<i32>, String> {
    let mut numbers = Vec::new();

    for token in input.split_whitespace() {
        match token.parse::<i32>() {
            Ok(n) => numbers.push(n),
            Err(_) => return Err(format!("Niepoprawna liczba: {}", token)),
        }
    }

    if numbers.is_empty() {
        Err("Brak liczb do przeanalizowania.".to_string())
    } else {
        Ok(numbers)
    }
}

/// Zwraca krotkę (liczba elementów, minimum, maksimum, suma)
pub fn summarize_numbers(numbers: &[i32]) -> (usize, i32, i32, i32) {
    let count = numbers.len();
    let mut min = numbers[0];
    let mut max = numbers[0];
    let mut sum = 0;

    for &n in numbers {
        if n < min {
            min = n;
        }
        if n > max {
            max = n;
        }
        sum += n;
    }

    (count, min, max, sum)
}

/// Buduje cztery linie raportu na podstawie przekazanych liczb.
pub fn describe_numbers(numbers: &[i32]) -> Vec<String> {
    let (count, min, max, sum) = summarize_numbers(numbers);

    let numbers_as_text = numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    vec![
        format!("Liczby ({}): {}", count, numbers_as_text),
        format!("Minimum: {}", min),
        format!("Maksimum: {}", max),
        format!("Suma: {}", sum),
    ]
}

/// Odpowiada za pełną analizę: parsowanie wejścia i przygotowanie raportu.
pub fn run_analysis(line: &str) -> Result<Vec<String>, String> {
    let numbers = parse_numbers(line)?;
    Ok(describe_numbers(&numbers))
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