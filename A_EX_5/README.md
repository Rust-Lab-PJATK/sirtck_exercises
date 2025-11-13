# Raport z liczb (`A_EX_5`)

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `A_EX_5`.
2. Na swoim branchu zaimplementuj brakujące fragmenty w `src/main.rs`, zastępując wszystkie wywołania `todo!()`.
3. Stwórz Pull Request do brancha głównego z prefiksem `A_EX_5` i poczekaj na wynik automatycznej oceny.

## Instrukcja zadania
- Program wczytuje jedną linię ze standardowego wejścia; oczekujemy liczb całkowitych oddzielonych białymi znakami (spacje, tabulatory, nowe linie).
- Funkcja `parse_numbers` przyjmuje surową linię tekstu i zwraca `Result<Vec<i32>, String>`:
  - użyj `split_whitespace`, aby przejść po tokenach,
  - każdy token parsuj przez `str::parse::<i32>()`; przy błędzie zwróć `Err(format!("Niepoprawna liczba: {}", token))`,
  - jeśli po przetworzeniu nie ma żadnych liczb, zwróć `Err("Brak liczb do przeanalizowania.".to_string())`.
- Funkcja `summarize_numbers` przyjmuje referencję do wektora i zwraca krotkę `(usize, i32, i32, i32)` odpowiadającą kolejno: liczbie elementów, minimum, maksimum i sumie.
- Funkcja `describe_numbers` buduje raport (`Vec<String>`) w następującej kolejności:
  1. `format!("Liczby ({}): {}", count, numbers_as_text)` – `numbers_as_text` zawiera liczby oddzielone pojedynczą spacją w oryginalnej kolejności,
  2. `format!("Minimum: {}", min)`,
  3. `format!("Maksimum: {}", max)`,
  4. `format!("Suma: {}", sum)`.
- Funkcja `run_analysis` spina całość: wywołuje `parse_numbers`, przekazuje wynik do `describe_numbers` i zwraca `Result<Vec<String>, String>`.
- Funkcja `main` czyta linię za pomocą `std::io::stdin().read_line`, a następnie:
  - w przypadku błędu wejścia wypisuje `Nie udało się odczytać danych.` i kończy działanie,
  - w przeciwnym razie wywołuje `run_analysis`; dla `Ok(raport)` wypisuje wszystkie linie po kolei, dla `Err(komunikat)` wypisuje komunikat błędu dokładnie w takiej postaci, jak został zwrócony.
- Unikaj `unwrap()` na danych pochodzących od użytkownika – wszystkie błędy propagujemy przez `Result`.

Przykład

Wejście: `10 -5 3 3`

Wyjście:
```
Liczby (4): 10 -5 3 3
Minimum: -5
Maksimum: 10
Suma: 11
```

## Wprowadzenie do nowych pojęć
- `split_whitespace`: metoda na `&str`, która dzieli tekst po dowolnych białych znakach i zwraca iterator po segmentach.

  ```rust
  let line = "10   -5\t3";
  for token in line.split_whitespace() {
      println!("{token}");
  }
  // wypisze kolejno: 10, -5, 3
  ```

- Parsowanie liczb z walidacją przez `map` i `collect::<Result<...>>`:

  ```rust
  let parsed: Result<Vec<i32>, _> = line
      .split_whitespace()
      .map(|token| token.parse::<i32>())
      .collect();

  match parsed {
      Ok(numbers) => println!("Mam liczby: {:?}", numbers),
      Err(_) => println!("Jedno z pól nie było poprawną liczbą całkowitą"),
  }
  ```

- Łączenie liczb w pojedynczy napis: najpierw zamień liczby na `String`, potem użyj `join(" ")`.

  ```rust
  let numbers = vec![10, -5, 3];
  let joined = numbers
      .iter()
      .map(|n| n.to_string())
      .collect::<Vec<_>>()
      .join(" ");
  assert_eq!(joined, "10 -5 3");
  ```

- `Iterator::min`, `Iterator::max` i `Iterator::sum`: działają na iteratorach po referencjach; `copied()` pozwala uzyskać wartości typu `i32`.

  ```rust
  let numbers = vec![10, -5, 3];
  let min = numbers.iter().copied().min().unwrap();
  let max = numbers.iter().copied().max().unwrap();
  let sum: i32 = numbers.iter().sum();
  ```

- Krotki (`(usize, i32, i32, i32)`): możesz zwrócić kilka wartości naraz, a później je rozpakować.

  ```rust
  let summary = (numbers.len(), min, max, sum);
  let (count, min_value, max_value, total) = summary;
  ```

## Ekstra podpowiedzi
- Zacznij od `parse_numbers` i przygotuj dokładne komunikaty błędów – testy porównują całe napisy.
- `summarize_numbers` może używać jednej pętli `for`, aby policzyć sumę oraz aktualizować minimum i maksimum.
- Przy formacie `Liczby (...)` dbaj o pojedyncze spacje między liczbami i brak spacji na końcu.
- `run_analysis` powinno wywoływać `parse_numbers` tylko raz – wykorzystaj wynik zamiast ponownie dzielić linię.

## Uwaga
Modyfikuj wyłącznie pliki w katalogu `src/` (możesz dodawać własne moduły pomocnicze). Zmiany w pozostałych częściach projektu mogą spowodować odrzucenie rozwiązania.
