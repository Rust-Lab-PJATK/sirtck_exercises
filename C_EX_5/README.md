# Abstrakcyjny silnik automatu komórkowego (`C_EX_5`)

## Jak oddać rozwiązanie
- Stwórz branch z prefixem `C_EX_5`.
- Na swoim branchu zaimplementuj wszystkie brakujące fragmenty w pliku `src/lib.rs`, zastępując wywołania `todo!()`.
- Stwórz Pull Request do głównego repozytorium z prefiksem `C_EX_5` w nazwie.
- Zaczekaj na wynik automatycznej oceny albo review.

## Instrukcja zadania
- Zaimplementuj `Grid<Cell>` przechowującą szerokość, wysokość oraz spłaszczone dane. Konstruktor `Grid::new(width, cells)` powinien odrzucić zerową szerokość, pusty wektor oraz długość niepodzielną przez `width`, zwracając `GridBuildError`.
- Dodaj metody dostępu: `width`, `height`, `cells`, `row` (pożyczony wycinek wiersza) oraz `replace_cells`, która podmienia bufor po uprzednim sprawdzeniu długości.
- Przygotuj `RowSegment<'a, Cell>` opisujący ciąg komórek od indeksu `offset`. Udostępnij metody `offset`, `cells`, `len`, `contains`, `at` oraz `relative(focus_x, dx)`, które wykorzystasz w sąsiedztwie.
- Zbuduj `Neighborhood<'a, Cell>` przechowujące `north`, `center`, `south` w formie `Option<RowSegment<'a, Cell>>` oraz `focus_x`. Zapewnij metody `focus_x`, `center`, `north`, `south`, `focus` i `get(dx, dy)`, które zwracają referencję do wybranej komórki (`dx, dy` w zakresie -1..=1).
- Dodaj prostą strukturę `Position { x, y }` wykorzystywaną przy wywołaniu reguły.
- Zdefiniuj trait `Rule<Cell>` z metodą `apply(&mut self, Position, Neighborhood<'_, Cell>) -> Result<Cell, Self::Error>`, która może zwrócić błąd domenowy.
- Zaimplementuj strukturę `Automaton<Cell, R>` przechowującą bieżącą siatkę i regułę. Zapewnij `new`, `grid`, `step` (jedno przejście reguły po całej siatce) oraz `step_many`, które wykonuje wiele kroków z krótszym kodem.
- Dodaj blanket-implementację `Rule<Cell>` dla closur `FnMut(Position, Neighborhood<'_, Cell>)` z wykorzystaniem `for<'a>`, aby testy mogły dostarczać reguły jako funkcje.
- Upewnij się, że `Automaton::step` korzysta z pożyczonych okien (`Grid::neighborhood`) i nie niszczy bieżących danych przed zakończeniem całego przebiegu.

## Nowe pojęcia i przypomnienia
- **Pożyczone okna wierszy (`&[T]`)**  
  Zamiast kopiować dane komórek, operujemy na referencjach do fragmentów wektora. Przydatne są operacje na indeksach początkowych i końcowych:
  ```rust
  fn window<'a, T>(row: &'a [T], x: usize) -> &'a [T] {
      let start = x.saturating_sub(1);
      let end = usize::min(x + 1, row.len() - 1);
      &row[start..=end]
  }
  ```
  W zadaniu `RowSegment` ma przechowywać właśnie takie pożyczone wycinki, dzięki czemu reguła ogląda sąsiedztwo bez alokacji.

- **`usize::checked_add_signed` / praca z przesunięciami**  
  Konwersja `dx: isize` na indeks `usize` wymaga zabezpieczenia przed niedozwolonymi wartościami. Rust udostępnia metodę `checked_add_signed`, która zwróci `None`, gdy wynik wychodzi poza zakres:
  ```rust
  let target = focus_x.checked_add_signed(dx).and_then(|x| segment.at(x));
  ```
  Dzięki temu w `Neighborhood::get` możemy bezpiecznie obliczać sąsiadów nawet na krawędziach siatki.

- **Ponowne użycie bufora wynikowego**  
  Przy każdej ewolucji można przygotować wektor na wynik z odpowiednią pojemnością, a dopiero po przejściu całej siatki podmienić go w `Grid`:
  ```rust
  let mut next = Vec::with_capacity(grid.cells().len());
  for y in 0..grid.height() {
      for x in 0..grid.width() {
          let view = grid.neighborhood(Position { x, y });
          next.push(rule.apply(Position { x, y }, view)?);
      }
  }
  grid.replace_cells(next)?;
  ```
  Dzięki temu unikamy wielokrotnych alokacji podczas każdego kroku automatu.

## Ekstra podpowiedzi
- W `Grid::neighborhood` przygotuj wspólny `start_x` i `end_x` dla wszystkich trzech wierszy – pozwoli to zwrócić spójne fragmenty nawet przy brakujących sąsiadach (zwróć `None` dla `north`/`south` na krawędziach).
- Rząd `RowSegment` przechowuje `offset`, więc `relative` może obliczyć indeks poprzez `offset + (focus_x - offset + dx)` bez kopiowania danych.
- `Automaton::step_many` warto zbudować w oparciu o `step`, co upraszcza propagację błędów (`?`).
- Jeśli korzystasz z `thiserror`, pamiętaj o dodaniu go w `Cargo.toml`; alternatywnie możesz samodzielnie zaimplementować `Display` i `Error` dla `GridBuildError`.
- Testy oczekują, że `Rule` można przekazać jako closurę – nie zapomnij o blanket-impl z `for<'a>`.

## Uwaga
- Modyfikuj tylko pliki w katalogach `exercices/C_EX_5` i `tasks/T_C_EX_5`. Instrukcję realizuj w `src/lib.rs` w miejscach oznaczonych `todo!()`.
- Po zakończeniu prac uruchom `cargo test` lokalnie, aby upewnić się, że wszystkie scenariusze są pokryte.
