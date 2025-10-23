# Cykliczny bufor z iteratorem (`C_EX_2`)

## Jak oddać rozwiązanie
- Stwórz branch z prefixem `C_EX_2`.
- Na swoim branchu zaimplementuj brakujące fragmenty w pliku `src/lib.rs`, zastępując wszystkie wywołania `todo!()`.
- Uruchom lokalnie testy (`cargo test`) aby upewnić się, że implementacja spełnia wymagania.
- Stwórz Pull Request do głównego repozytorium z prefiksem `C_EX_2` w nazwie.
- Poczekaj na wynik automatycznej oceny lub review.

## Instrukcja zadania
- Zaimplementuj strukturę `RingBuffer<T>` o stałej pojemności. Przechowuj w niej ostatnie `capacity` elementów i zwracaj nadpisane wartości w metodzie `push`.
- Konstruktor `RingBuffer::new` powinien zwracać `Result<Self, BufferError>` i odrzucać `capacity == 0` (użyj `thiserror` do zdefiniowania `BufferError`).
- Udostępnij metody `len`, `capacity`, `is_empty`, `is_full` oraz `from_iter` (przyjmuje `IntoIterator` i ładuje dane do bufora, nadpisując najstarsze elementy).
- Przygotuj iterator `RingIter<'a, T>` zwracany przez `RingBuffer::iter` oraz implementację `IntoIterator` dla `&RingBuffer<T>`. Iterator ma przechodzić od najstarszego do najnowszego elementu i wspierać również iterację od końca (`DoubleEndedIterator`).
- Zaimplementuj `iter_recent(&self, count)` ograniczający liczbę zwracanych elementów, `into_sorted_vec_by` (klonuje i sortuje dane przy pomocy przekazanego komparatora) oraz `find_last` zwracający referencję do ostatniego elementu spełniającego predykat.
- Unikaj alokacji podczas każdego `push`. Bufor powinien zaalokować pamięć tylko raz przy tworzeniu struktury.

## Nowe pojęcia i przypomnienia
- **Generyczne typy i Trait Bounds** – zadeklaruj `RingBuffer<T>` oraz metody ograniczając typ `T` tylko tam, gdzie to konieczne (np. `T: Clone` w `into_sorted_vec_by`). Przykład deklaracji metody z ograniczeniami:

  ```rust
  pub fn map_into<U, F>(&self, mut f: F) -> Vec<U>
  where
      F: FnMut(&T) -> U,
  {
      self.iter().map(&mut f).collect()
  }
  ```

- **Lifetimes w iteratorach** – aby zwracać referencje, iterator musi znać czas życia danych. Pola struktury `RingIter<'a, T>` mogą przechowywać odwołania do wewnętrznego wektora bufora:

  ```rust
  pub struct RingIter<'a, T> {
      slots: &'a [Option<T>],
      // indeksy i liczniki potrzebne do przejścia cyklicznego
  }
  ```

- **`thiserror`** – prosty sposób na implementację `std::error::Error` i czytelnych komunikatów. Wystarczy ozdobić enum atrybutem `#[derive(Error)]` i zdefiniować komunikaty:

  ```rust
  use thiserror::Error;

  #[derive(Debug, Error, PartialEq, Eq)]
  pub enum BufferError {
      #[error("capacity must be greater than zero")]
      ZeroCapacity,
  }
  ```

- **`Iterator` i `DoubleEndedIterator`** – implementując oba traity pozwalasz na przechodzenie w przód (`next`) i w tył (`next_back`). To umożliwi wykorzystanie bufora w idiomatycznych konstrukcjach (`for`, `collect`, `rev()`).

- **`IntoIterator` dla referencji** – implementując `IntoIterator` dla `&RingBuffer<T>` pozwolisz na pisanie `for item in &buffer { ... }`. To popularny wzorzec w bibliotekach standardowych.

## Ekstra podpowiedzi
- Do przechowywania danych dobrze sprawdza się `Vec<Option<T>>`: puste sloty pozostają `None`, a zajęte przechowują `Some(T)`. Indeks najstarszego elementu możesz wyliczyć z `head` i `len`.
- Przy aktualizacji `head` pamiętaj o operacji modulo (`(head + 1) % capacity`).
- `iter_recent` może wykorzystać `RingIter::rev()` i `take(count)` do ograniczenia liczby elementów, a następnie odwrócić kolejność z powrotem.
- Testy oczekują, że kopiowanie danych w `into_sorted_vec_by` nastąpi dopiero po wywołaniu tej metody – nie duplikuj przechowywanych elementów w innych operacjach.

## Uwaga
- Modyfikuj wyłącznie pliki w katalogu `exercices/C_EX_2` i implementuj kod w miejscach oznaczonych `todo!()`.
- Zostaw strukturę projektu bez zmian (Cargo.toml, moduły), a dodatkowe pliki dodawaj tylko jeśli są potrzebne do rozwiązania zadania.
