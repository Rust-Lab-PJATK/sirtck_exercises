# Histogram liter (`A_EX_2`)

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `A_EX_2`.
2. Na swoim branchu zaimplementuj wymagane funkcje w pliku `src/main.rs`, zastępując miejsce oznaczone `todo!()`.
3. Stwórz Pull Request do brancha głównego z prefiksem `A_EX_2`.
4. Po akceptacji PR lub uruchomieniu automatu otrzymasz informację zwrotną.

## Instrukcja zadania
- Program czyta jedną linię tekstu ze standardowego wejścia (stdin). Pusta linia lub brak liter po przetworzeniu powinny skutkować komunikatem `Brak danych`.
- Litery traktujemy bez rozróżniania wielkości: `A` i `a` liczą się razem. Bierz pod uwagę tylko litery ASCII (`a`–`z`); wszystkie inne znaki (spacje, cyfry, interpunkcję, znaki diakrytyczne) ignoruj.
- Wynik należy wypisać w kolejnych liniach w formacie `litera: liczba`, posortowany alfabetycznie (od `a` do `z`).
- Wykorzystaj `Result` (i `match`) do obsługi błędów wejścia, a logikę zliczania umieść w wydzielonych funkcjach tak, by można je było osobno przetestować.
- Przygotowany plik `src/main.rs` zawiera szkic pomocniczych funkcji (`normalize_letters`, `build_histogram`, `format_histogram`, `run_from_str`). Uzupełnij je, zamiast pisać własne interfejsy.

## Wprowadzenie do nowych pojęć
- `Result<T, E>`: typ służący do zwracania wyniku (`Ok(T)`) albo błędu (`Err(E)`). W tym zadaniu używamy `Result<Vec<char>, String>` – sukces to lista liter, błąd to komunikat tekstowy.

  ```rust
  let wynik: Result<Vec<char>, String> = normalize_letters("Ala");
  match wynik {
      Ok(letters) => println!("{:?}", letters),
      Err(message) => println!("Błąd: {message}"),
  }
  ```

- `match`: instrukcja dopasowania wzorców. W `run_from_str` przyda się `match normalize_letters(...)` do rozróżnienia `Ok` i `Err`.

  ```rust
  match normalize_letters("123!") {
      Ok(_) => println!("Są litery"),
      Err(e) => println!("Brak danych: {e}"),
  }
  ```

- `BTreeMap<K, V>`: uporządkowana mapa z biblioteki standardowej (`std::collections`). Każde wstawienie `counts.entry(litera).and_modify(...).or_insert(...)` przechowuje klucze w kolejności rosnącej.

  ```rust
  use std::collections::BTreeMap;

  let mut counts = BTreeMap::new();
  for ch in ['a', 'b', 'a'] {
      counts.entry(ch).and_modify(|c| *c += 1).or_insert(1);
  }
  assert_eq!(counts.get(&'a'), Some(&2));
  ```

- Przetwarzanie znaków: metoda `char::is_ascii_alphabetic()` sprawdza, czy znak należy do alfabetu łacińskiego, a `char::to_ascii_lowercase()` zwraca mały odpowiednik litery. Do wycinania białych znaków użyj `str::trim()`.

  ```rust
  let input = " Rust! ";
  let trimmed = input.trim();
  let letters: Vec<char> = trimmed
      .chars()
      .map(|c| c.to_ascii_lowercase())
      .filter(|c| c.is_ascii_alphabetic())
      .collect();
  assert_eq!(letters, vec!['r', 'u', 's', 't']);
  ```

- Czytanie wejścia: `io::stdin().read_line(&mut buffer)` dopisuje wczytaną linię do bufora. Funkcje zadania powinny pracować na `&str`, więc po wczytaniu przekaż referencję `&buffer`.

  ```rust
  use std::io;

  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).expect("czytanie stdin");
  if buffer.trim().is_empty() {
      println!("Brak danych");
  }
  ```

## Ekstra podpowiedzi
- Do przefiltrowania znaków możesz użyć metody `char::is_ascii_alphabetic()` razem z `to_ascii_lowercase()`.
- `BTreeMap` automatycznie przechowuje klucze w kolejności; wystarczy inkrementować liczniki i potem przeiterować mapę.
- W `normalize_letters` pomocne będzie `trim()` oraz sprawdzenie, czy wektor z literami nie jest pusty, zanim zwrócisz `Ok`.
- Funkcję `run_from_str` zbuduj warstwowo: normalizacja → histogram → formatowanie. Każdy krok zwróci Ci dane do następnego kroku.

## Uwaga
Podczas rozwiązywania zadania modyfikuj tylko wskazane miejsca. Zmiany w pozostałych częściach projektu mogą spowodować odrzucenie rozwiązania.
