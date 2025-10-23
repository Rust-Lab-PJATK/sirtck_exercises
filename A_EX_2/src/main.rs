use std::collections::BTreeMap;
use std::io;

/// Zamienia wejściowy tekst na wektor znaków zawierający tylko litery alfabetu.
/// Zwraca `Err("Brak danych")`, gdy po przetworzeniu brak liter.
pub fn normalize_letters(input: &str) -> Result<Vec<char>, String> {
    let normalized: Vec<_> = input.trim()
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    if normalized.is_empty() {
        Err(String::from("Brak danych"))
    } else {
        Ok(normalized)
    }
}

/// Buduje histogram wystąpień liter na podstawie wektora znaków.
pub fn build_histogram(letters: &[char]) -> BTreeMap<char, usize> {
    let mut histogram = BTreeMap::new();
    for char in letters {
        histogram.entry(*char).and_modify(|e| *e += 1).or_insert(1);
    }
    histogram
}

/// Formatuje histogram do wypisania na stdout jako linie `litera: liczba`.
pub fn format_histogram(counts: &BTreeMap<char, usize>) -> Vec<String> {
    let mut histogram = Vec::new();
    for(char, count) in counts {
        histogram.push(format!("{char}: {count}"))
    }
    histogram
}

/// Główna logika: przygotowuje wynik do wypisania lub zwraca komunikat błędu.
pub fn run_from_str(input: &str) -> Result<Vec<String>, String> {
    match normalize_letters(input) {
        Ok(letters) => Ok(format_histogram(&build_histogram(&letters))),
        Err(msg) =>  Err(msg)
    }
}

pub fn main() {
    let mut buffer = String::new();

    if io::stdin().read_line(&mut buffer).is_err() {
        println!("Brak danych");
        return;
    }

    match run_from_str(&buffer) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(message) => println!("{}", message),
    }
}
