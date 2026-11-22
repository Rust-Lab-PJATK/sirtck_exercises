use std::collections::BTreeMap;
use std::io;

/// Zamienia wejściowy tekst na wektor znaków zawierający tylko litery alfabetu.
/// Zwraca `Err("Brak danych")`, gdy po przetworzeniu brak liter.
pub fn normalize_letters(input: &str) -> Result<Vec<char>, String> {
    let mut litery = Vec::new();

    for i in input.chars() {
        if i.is_alphabetic() {
            litery.push(i.to_ascii_lowercase());
        }
    }

    if litery.is_empty() {
        Err("Brak danych".to_string())
    } else {
        Ok(litery)
    }
}

/// Buduje histogram wystąpień liter na podstawie wektora znaków.
pub fn build_histogram(letters: &[char]) -> BTreeMap<char, usize> {
    let mut histogram = BTreeMap::new();

    for &litera in letters {
        if histogram.contains_key(&litera) {
            let liczba = histogram.get_mut(&litera).unwrap();
            *liczba += 1;
        } else {
            histogram.insert(litera, 1);
        }
    }

    histogram
}

/// Formatuje histogram do wypisania na stdout jako linie `litera: liczba`.
pub fn format_histogram(counts: &BTreeMap<char, usize>) -> Vec<String> {
    let mut wynik = Vec::new();

    for (litera, liczba) in counts {
        wynik.push(format!("{}: {}", litera, liczba));
    }

    wynik
}

/// Główna logika: przygotowuje wynik do wypisania lub zwraca komunikat błędu.
pub fn run_from_str(input: &str) -> Result<Vec<String>, String> {
    let litery = normalize_letters(input)?;
    let liczba = build_histogram(&litery);
    Ok(format_histogram(&liczba))
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