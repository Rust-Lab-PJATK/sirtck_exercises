# Checklist tematów (`A_EX_4`)

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `A_EX_4`.
2. Na swoim branchu zaimplementuj brakujące fragmenty w `src/main.rs`, zastępując wszystkie wywołania `todo!()`.
3. Stwórz Pull Request do brancha głównego z prefiksem `A_EX_4` i poczekaj na wynik automatycznej oceny.

## Instrukcja zadania
- Program wczytuje jedną linię tekstu ze standardowego wejścia. Ta linia zawiera listę tematów oddzielonych przecinkami (np. `powtórka zmiennych, instrukcje warunkowe, pętle`).
- Funkcja `collect_topics` ma:
  - przyciąć każdy fragment (`trim`), odfiltrować puste wpisy i zamienić je na `String`,
  - zwrócić `Ok(Vec<String>)`, jeśli po filtracji coś zostało,
  - zwrócić `Err("Brak tematów do przećwiczenia.")`, gdy linia była pusta lub po usunięciu pustych elementów nie ma nic do wypisania.
- Funkcja `format_checklist` buduje numerowaną listę wierszy w formacie `"{}. [ ] {}"` z numeracją od 1. Wykorzystaj do tego pętlę `for` po `items.iter()` lub adapter `enumerate`.
- Funkcja `build_summary` policzy dwie rzeczy:
  - liczbę tematów (`items.len()`),
  - łączną liczbę znaków w tematach z pominięciem spacji (`topic.chars().filter(|c| !c.is_whitespace()).count()`),
  i zwróci wiersz `format!("Podsumowanie: {} elementów, {} znaków (bez spacji)", count, char_count)`.
- Funkcja `generate_report` łączy wszystkie powyższe kroki: wywołuje `collect_topics`, następnie tworzy checklistę i dopisuje linię podsumowania. Zwraca `Result<Vec<String>, String>`, tak aby błędy mogły zostać wypisane bezpośrednio w `main`.
- Funkcja `main` czyta linię ze standardowego wejścia i wyświetla wynik:
  - jeśli `generate_report` zwróci `Ok(linie)`, każdą linię wypisz osobno z `println!`,
  - jeśli pojawi się `Err(komunikat)`, wypisz komunikat błędu dokładnie tak, jak został zwrócony.

## Wprowadzenie do nowych pojęć
- `for item in items.iter()`: iteracja po elementach wektora bez ich przenoszenia. Dzięki temu możesz korzystać z referencji do każdego elementu.

  ```rust
  let items = vec!["a", "b", "c"];
  for item in items.iter() {
      println!("{item}");
  }
  ```

- `enumerate`: adapter iteratora, który dokłada numer porządkowy (liczony od zera). Idealny do numerowania linii checklisty.

  ```rust
  let checklist: Vec<String> = items
      .iter()
      .enumerate()
      .map(|(index, value)| format!("{}. [ ] {value}", index + 1))
      .collect();
  ```

- `map`, `filter`, `collect::<Vec<_>>()`: kombinacja adapterów pozwalająca najpierw przetworzyć dane (`map`), odsiać niepotrzebne elementy (`filter`), a na końcu zebrać wynik w nowy `Vec`.

  ```rust
  let cleaned: Vec<String> = line
      .split(',')
      .map(|chunk| chunk.trim())
      .filter(|chunk| !chunk.is_empty())
      .map(|chunk| chunk.to_string())
      .collect();
  ```

- Liczenie znaków w `String`: `chars()` zwraca iterator po znakach Unicode, a `.filter(|c| !c.is_whitespace()).count()` pomaga policzyć tylko te znaki, które nie są spacjami.

  ```rust
  let without_spaces = topic.chars().filter(|c| !c.is_whitespace()).count();
  ```

- Sumowanie liczb z iteratora: `iter().map(...).sum::<usize>()` pozwala zsumować wyniki kolejnych obliczeń.

  ```rust
  let total_chars: usize = items
      .iter()
      .map(|topic| topic.chars().filter(|c| !c.is_whitespace()).count())
      .sum();
  ```

## Ekstra podpowiedzi
- Najpierw zadbaj o poprawne działanie `collect_topics`. Reszta funkcji będzie prostsza, kiedy masz gotowy `Vec<String>` z danych wejściowych.
- Numerację checklisty możesz zrobić pętlą `for (index, topic) in items.iter().enumerate() { ... }`. Pamiętaj o zamianie numeru z 0-based na 1-based.
- Zwracaj dokładnie te komunikaty tekstowe, które opisano w instrukcji – testy porównują je znak w znak.
- Do konstruowania raportu trzymaj się sekwencji: zbierz tematy → zbuduj checklistę → dodaj linię podsumowania.

## Uwaga
Modyfikuj tylko pliki w katalogu `src/` (oraz ewentualnie dodane przez siebie moduły). Zmiany w pozostałych częściach projektu mogą spowodować odrzucenie rozwiązania.
