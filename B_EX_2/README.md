# Podsumowanie zadan (`B_EX_2`)

## Jak oddac rozwiazanie
1. Stworz branch z prefiksem `B_EX_2`.
2. Na tym branchu zastep wszystkie wywolania `todo!()` w `src/lib.rs`, nie zmieniajac publicznego API.
3. Otworz Pull Request do galezi glownej z tym samym prefiksem `B_EX_2`.
4. Zaczekaj na informacje zwrotna od automatu albo recenzenta.

## Opis zadania
- Czytaj dane ze standardowego wejscia. Kazda niepusta linia ma format `tytul | priorytet | status`. Usun dodatkowe spacje wokol pol.
- Akceptuj tylko priorytety `low`, `medium`, `high` (niezaleznie od wielkosci liter) oraz statusy `todo`, `in_progress`, `done` (takze case-insensitive). Inne wartosci to blad.
- Puste linie ignoruj. Jesli po odrzuceniu pustych linii nie zostanie zadne poprawne zadanie, zwroc blad `Brak zadan`.
- Gdy linia ma zly format (brak separatorow, za malo lub za duzo pol), zwroc `Niepoprawny format linii: <oryginalna linia>`.
- Nieznany priorytet sygnalizuj `Nieznany priorytet: <wartosc>`, a nieznany status `Nieznany status: <wartosc>`. Uzyj do tego `TaskParseError`.
- Po sparsowaniu zbuduj raport, w ktorym kazdy status wystepuje dokladnie raz i w kolejnosci `TODO`, `IN_PROGRESS`, `DONE`. Dla kazdego statusu policz ile zadan ma priorytety `high`, `medium`, `low`.
- Wypisz (lub zwroc) linie w stylu `TODO: 2 (high: 1, medium: 1, low: 0)`. Nawet brakujace priorytety musza pojawic sie z wartoscia `0`.
- Glowna logika powinna pozostac w module bibliotecznym. Plik `main.rs` juz wywoluje `run_from_reader`, wiec nie modyfikuj jego struktury.

Elementy do zaimplementowania:
- `domain::Priority::label`, `domain::Status::label` oraz odpowiadajace im implementacje `fmt::Display`.
- `fmt::Display` dla `domain::TaskParseError` (z dokladnymi komunikatami wymienionymi powyzej).
- `domain::Task::from_str`, ktora przycina wartosci, dzieli linie i zwraca odpowiednie bledy.
- `StatusSummary::total`, `parse_tasks`, `summarize_by_status`, `format_summary`, `run_from_reader`.

## Nowe pojecia
- **Podzial na `lib.rs` i `main.rs`**  
  Masz jednoczesnie crate biblioteczny (logika) i binarny (wejscie/wyjscie). Binarka powinna korzystac z API biblioteki zamiast duplikowac kod.

  ```rust
  // src/lib.rs
  pub fn run_from_reader<R: BufRead>(reader: R) -> Result<Vec<String>, TaskParseError> {
      let tasks = parse_tasks(&tekst)?;
      // ...
  }

  // src/main.rs
  fn main() {
      let stdin = io::stdin();
      if let Err(err) = b_ex_2::run_from_reader(stdin.lock()) {
          eprintln!("{err}");
      }
  }
  ```

- **`std::str::FromStr` do wlasnego parsowania**  
  Dzieki `FromStr` mozesz wygodnie uzyc operatora `?` i konwersji `str::parse`.

  ```rust
  impl FromStr for Priority {
      type Err = TaskParseError;

      fn from_str(raw: &str) -> Result<Self, Self::Err> {
          match raw.trim().to_ascii_lowercase().as_str() {
              "high" => Ok(Priority::High),
              other => Err(TaskParseError::InvalidPriority(other.into())),
          }
      }
  }
  ```

- **`fmt::Display` dla komunikatow dla uzytkownika**  
  Zamiast `Debug` przygotuj czytelne teksty, ktore trafia na stdout lub stderr.

  ```rust
  impl fmt::Display for TaskParseError {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          match self {
              TaskParseError::InvalidStatus(value) => write!(f, "Nieznany status: {value}"),
              _ => write!(f, "Brak zadan"),
          }
      }
  }
  ```

- **`BufRead` i `lines()`**  
  Zablokuj `stdin`, iteruj po liniach, a w testach mozesz wstrzyknac `Cursor` z tekstem.

  ```rust
  use std::io::{self, BufRead};

  fn read_input() -> io::Result<String> {
      let mut buffer = String::new();
      io::stdin().lock().read_to_string(&mut buffer)?;
      Ok(buffer)
  }
  ```

- **`BTreeMap` do uporzadkowanej agregacji**  
  Klucze sa sortowane, co ulatwia przechodzenie po `Priority::all()`.

  ```rust
  let mut counts = BTreeMap::new();
  counts.entry(Priority::High).or_insert(0);
  ```

## Ekstra podpowiedzi
- Normalizuj wielkosc liter raz, np. przez `to_ascii_lowercase`, zamiast powtarzac warunki.
- Korzystaj z `Priority::all()` oraz `Status::all()`, zeby spelnic wymagany porzadek w jednym miejscu.
- Pozostan przy jednym typie bledu (`TaskParseError`) i propaguj go operatorem `?`.
- Testy beda odpalac API biblioteki, dlatego rozbij `parse_tasks` i `summarize_by_status` na male, latwe do sprawdzenia funkcje.

## Uwaga
Nie zmieniaj plikow ani interfejsow spoza miejsc oznaczonych `todo!()`. Zmiany poza zakresem zadania moga spowodowac negatywna ocene. Jesli potrzebujesz ponownie uruchomic automat, popros administratora.
