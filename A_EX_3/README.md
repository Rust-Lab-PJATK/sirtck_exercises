# Sumator CLI (`A_EX_3`)

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `A_EX_3`.
2. Na swoim branchu zaimplementuj brakujące fragmenty w `src/main.rs`, zastępując wywołania `todo!()`.
3. Stwórz Pull Request do brancha głównego z prefiksem `A_EX_3` i poczekaj na wynik automatycznej oceny.

## Instrukcja zadania
- Program ma działać w pętli: czytaj kolejne linie ze standardowego wejścia aż do momentu, gdy użytkownik wpisze `koniec`.
- Każdą linię przekaż do funkcji `parse_line`. Funkcja powinna zwrócić:
  - `Ok(Some(liczba))`, gdy po przycięciu białych znaków można ją sparsować jako liczbę całkowitą (`i32`);
  - `Ok(None)`, gdy użytkownik wpisał dokładnie `koniec`;
  - `Err(komunikat)`, gdy linia jest pusta albo nie przypomina liczby (wtedy wypisz komunikat i powtórz pętlę bez zmiany sumy).
- Przygotuj dwa komunikaty błędów: dla pustej linii zwróć i wypisz `Wpisz liczbę lub 'koniec'.`, a dla pozostałych błędnych danych użyj `Niepoprawna liczba: {treść}` (gdzie `{treść}` to przycięte wejście).
- Po każdej poprawnej liczbie zaktualizuj sumę i wypisz `Aktualna suma: {wartość}`.
- Gdy `parse_line` zwróci `Ok(None)`, wypisz `Zamykam program. Suma: {wartość}` i zakończ pętlę.
- Funkcja `run_session` ma przyjmować dowolną sekwencję linii i zwracać listę komunikatów dokładnie w takiej kolejności, w jakiej powinny zostać wypisane. Testy korzystają z tej funkcji — upewnij się, że używa ona `parse_line` i zachowuje się tak samo jak interaktywna pętla.
- Nie używaj `unwrap()` do obsługi błędów pochodzących z danych użytkownika. Zamiast tego propaguj komunikaty tekstowe przez `Result`.

## Wprowadzenie do nowych pojęć
- `loop`, `break`, `continue`: nieskończona pętla, z której wychodzisz za pomocą `break`. `continue` pomija resztę iteracji i wraca na początek.

  ```rust
  let mut counter = 0;
  loop {
      counter += 1;
      if counter == 3 {
          continue; // pomijamy resztę tej iteracji
      }
      if counter > 5 {
          break; // kończymy pętlę
      }
  }
  ```

- `Option<T>`: typ reprezentujący wartość opcjonalną (`Some(T)` lub `None`). W tym zadaniu `Ok(None)` informuje, że trzeba zakończyć program.

  ```rust
  fn znajdz_parzysta(n: i32) -> Option<i32> {
      if n % 2 == 0 {
          Some(n)
      } else {
          None
      }
  }

  match znajdz_parzysta(6) {
      Some(value) => println!("Parzysta: {value}"),
      None => println!("Nieparzysta!"),
  }
  ```

- `str::trim()` i `Result` z `parse::<i32>()`: metoda `trim` usuwa białe znaki z początku i końca, a `parse` zwraca `Result`, które możesz obsłużyć dopasowaniem wzorca (`match`).

  ```rust
  let original = "  42  ";
  let trimmed = original.trim();
  match trimmed.parse::<i32>() {
      Ok(number) => println!("Liczba: {number}"),
      Err(_) => println!("Nie udało się sparsować liczby"),
  }
  ```

- `String::clear()` i ponowne użycie bufora: zamiast tworzyć nowy `String` w każdej iteracji pętli, wyczyść istniejący bufor i wczytaj do niego kolejną linię.

  ```rust
  use std::io;
  let mut stdin = io::stdin();
  let mut buffer = String::new();
  if stdin.read_line(&mut buffer).is_ok() {
      buffer.clear(); // usuwa poprzednią zawartość, ale zachowuje zaalokowaną pamięć
  }
  ```

## Ekstra podpowiedzi
- Traktuj puste linie tak samo jak inne niepoprawne dane: zwróć z `parse_line` błąd z komunikatem `Wpisz liczbę lub 'koniec'.`.
- Gdy chcesz wypisać linię w `run_session`, zapisz ją do `Vec<String>`; to dokładnie to, czego oczekują testy.
- Przetwarzaj dane krok po kroku: najpierw `parse_line`, potem `match` na wyniku i aktualizacja sumy. Dzięki temu testy jednostkowe będą mogły łatwo sprawdzać poszczególne sytuacje.
- Możesz zakończyć `run_session` natychmiast po `Ok(None)` – dodatkowe linie po `koniec` powinny być ignorowane.

## Uwaga
Modyfikuj tylko wskazane pliki: `src/main.rs`, ewentualnie dodatkowe moduły pomocnicze w katalogu `src/`. Zmiany w pozostałych częściach projektu mogą spowodować odrzucenie rozwiązania.
