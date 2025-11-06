# Funkcyjny łańcuch procesorów sygnału (`C_EX_4`)

## Jak oddać rozwiązanie
- Stwórz branch z prefixem `C_EX_4`.
- Na swoim branchu zaimplementuj brakujące fragmenty w pliku `src/lib.rs`, zastępując wszystkie wywołania `todo!()`.
- Stwórz Pull Request do głównego repozytorium z prefiksem `C_EX_4` w nazwie.
- Poczekaj na wynik automatycznej oceny lub review.

## Instrukcja zadania
- Zaimplementuj trait `SignalProcessor`, który pracuje na blokach próbek zdefiniowanych aliasem `Block<'a, Sample> = Cow<'a, [Sample]>`. Trait ma udostępniać metodę `process`, zwracającą `Result<Block<'a, Sample>, Error>`.
- Uzupełnij strukturę `Gain` tak, aby poprawnie przechowywała mnożnik i skalowała próbki. Jeżeli na wejściu otrzymasz `Cow::Owned`, zaktualizuj dane in-place zamiast alokować nowy `Vec`.
- Zaimplementuj `Chain` składający dwa procesory sekwencyjnie. Połączony pipeline powinien zwracać enum `ChainError`, który odróżnia błąd pierwszego i drugiego etapu.
- Uzupełnij procesor `Map`, który stosuje funkcję wyższego rzędu `FnMut(&Sample) -> Sample` do wyniku poprzedniego etapu. Dla `Cow::Borrowed` zaalokuj bufor wynikowy; dla `Cow::Owned` modyfikuj dane w miejscu.
- Uzupełnij procesor `Tap`, który wywołuje obserwatora `FnMut(&[Sample])` i przepuszcza dane bez zmian (zarówno w wariancie pożyczonym, jak i posiadanym).
- Rozszerz trait `ProcessorExt` metodami `then`, `map_samples` i `tap`, które budują opisane wyżej procesory. Dzięki temu użytkownik może pisać pipeline'y w stylu `gain.then(...)`.
- Dodaj blanket-implementację `SignalProcessor` dla `FnMut` z wyższym rangiem czasu życia (`for<'a>`), aby zwykłe closury mogły pełnić rolę etapów przetwarzania.

## Nowe pojęcia i przypomnienia
- **`std::borrow::Cow<'a, T>` – kopiuj przy zapisie (copy-on-write)**  
  `Cow` pozwala przekazywać dane, które czasem są tylko pożyczane (`Borrowed`), a czasem należą do struktury (`Owned`). Dzięki temu pipeline może uniknąć niepotrzebnych alokacji. Przykład pracy z `Cow`:
  ```rust
  fn to_uppercase(block: Cow<'_, [u8]>) -> Cow<'_, [u8]> {
      match block {
          Cow::Borrowed(slice) => Cow::Owned(slice.iter().map(u8::to_ascii_uppercase).collect()),
          Cow::Owned(mut vec) => {
              for byte in &mut vec {
                  *byte = byte.to_ascii_uppercase();
              }
              Cow::Owned(vec)
          }
      }
  }
  ```
  W zadaniu używamy tej techniki, aby modyfikować próbki tylko wtedy, gdy to konieczne.

- **Wyższe rzędy czasów życia (HRTB) w impl dla `FnMut`**  
  Implementacja `SignalProcessor` dla closur wymaga, by ta sama funkcja działała na blokach o dowolnym czasie życia. Rust zapisuje to jako `for<'a> FnMut(Block<'a, Sample>) -> ...`. Taki zapis gwarantuje, że `FnMut` nie przechowuje referencji krótszych niż przekazany blok.
  ```rust
  fn make_passthrough() -> impl for<'a> FnMut(Block<'a, f32>) -> Result<Block<'a, f32>, Infallible> {
      move |block| Ok(block)
  }
  ```

- **`std::convert::Infallible` jako wygodny typ błędu**  
  Gdy etap nie może się nie udać (`Gain` mnożący liczby), wybieramy `Infallible` jako typ błędu. Informuje to kompilator i czytelników, że operacja jest bezpieczna. Wykorzystaj `type Error = Infallible`, a w kodzie nie twórz wartości tego typu.

- **Kombinatory traitowe (`ProcessorExt`)**  
  Rozszerzenie traitu pozwala pisać fluent API: `processor.then(next).map_samples(...)`. Warto pamiętać, żeby metody zwracały nowe struktury posiadające poprzedni procesor – dlatego `ProcessorExt` wymaga `Sized`, a struktury jak `Chain` mają prywatne pola.

## Ekstra podpowiedzi
- Do mnożenia w `Gain` użyj dopasowania na `Cow`. Przykład in-place dla posiadanych danych:
  ```rust
  let factor = 0.5_f32;
  match block {
      Cow::Owned(mut vec) => {
          for sample in &mut vec {
              *sample *= factor;
          }
          Cow::Owned(vec)
      }
      Cow::Borrowed(slice) => {
          let scaled = slice.iter().map(|&sample| sample * factor).collect();
          Cow::Owned(scaled)
      }
  }
  ```
- `Chain` powinien ruszyć dopiero po sukcesie pierwszego etapu. Jeśli pierwszy etap zwróci `Err`, przechwyć go przez `map_err(ChainError::First)`.
- `Tap` może korzystać z `block.as_ref()` by uzyskać `&[Sample]` niezależnie od wariantu `Cow`.
- W testach znajdziesz przypadki, które oczekują zachowania pożyczonego bloku (`Cow::Borrowed`) po przejściu przez pipeline nie wykonujący modyfikacji – zwróć uwagę, czy nie alokujesz nowego bufora bez powodu.
- Modyfikuj wyłącznie pliki w katalogach `exercices/C_EX_4` oraz `tasks/T_C_EX_4`. Pozostała część projektu służy innym zadaniom.
