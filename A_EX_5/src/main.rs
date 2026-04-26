use std::io;

/// Parsuje liczby całkowite rozdzielone białymi znakami.
/// Zwraca komunikat `"Brak liczb do przeanalizowania."`, jeśli po przetworzeniu nie ma żadnych wartości.
pub fn parse_numbers(input: &str) -> Result<Vec<i32>, String> {

    ///
    let tokens: Result<Vec<i32>, _> = input
        .split_whitespace()
        .map(|t| t.parse::<i32>())
        .collect();

    match tokens {
        Ok(nums) if nums.is_empty() => Err("Brak liczb do przeanalizowania".to_string()),
        Ok(nums) => Ok(nums),
        Err(_) => Err("Niepoprawny format liczby".to_string()),
    }
}

/// Zwraca krotkę (liczba elementów, minimum, maksimum, suma) dla przekazanych liczb.
pub fn summarize_numbers(numbers: &[i32]) -> (usize, i32, i32, i32) {

    ///
    let min = numbers.iter().copied().min().unwrap();
    let max = numbers.iter().copied().max().unwrap();
    let sum: i32 = numbers.iter().sum();

    let summary = (numbers.len(), min, max, sum);


    summary
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
    let  numbers = parse_numbers(line)?;
    let analisys = describe_numbers(&numbers);

    Ok(analisys)

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
