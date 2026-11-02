use std::collections::BTreeMap;
use std::io;

/// Zamienia wejściowy tekst na wektor znaków zawierający tylko litery alfabetu.
/// Zwraca `Err("Brak danych")`, gdy po przetworzeniu brak liter.
pub fn normalize_letters(input: &str) -> Result<Vec<char>, String> {
    
    let res: Vec<char> = input.chars().filter(|x| x.is_alphabetic()).collect();
    if res.is_empty(){
        return Err("Brak danych".to_string());
    }
    Ok(res)
}

/// Buduje histogram wystąpień liter na podstawie wektora znaków.
pub fn build_histogram(letters: &[char]) -> BTreeMap<char, usize> {
    let mut hist: BTreeMap<char, usize> = BTreeMap::new();
    for &letter in letters.iter(){
        *hist.entry(letter).or_insert(0) += 1;
    }
    hist
}

/// Formatuje histogram do wypisania na stdout jako linie `litera: liczba`.
pub fn format_histogram(counts: &BTreeMap<char, usize>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for (c, n) in counts{
        res.push(format!("{}: {}", c, n))
    }
    res
}

/// Główna logika: przygotowuje wynik do wypisania lub zwraca komunikat błędu.
pub fn run_from_str(input: &str) -> Result<Vec<String>, String> {
    let norm = normalize_letters(input);
    if norm.is_err() {
        return Err(norm.err().expect("Brak danych"));
    }

    Ok(format_histogram(&build_histogram(&norm.unwrap())))
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
