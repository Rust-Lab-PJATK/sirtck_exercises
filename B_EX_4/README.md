# Raport z testów regresyjnych (`B_EX_4`)

## Jak oddać rozwiązanie
1. Stwórz branch z prefiksem `B_EX_4`.
2. Na tym branchu uzupełnij wszystkie miejsca oznaczone `todo!()` w `src/lib.rs`, nie zmieniaj publicznych interfejsów.
3. W module testowym (`#[cfg(test)] mod tests`) dodaj scenariusze zgodne z opisem zadania – testy muszą przechodzić po uruchomieniu `cargo test`.
4. Otwórz Pull Request do gałęzi głównej z tym samym prefiksem `B_EX_4` i poczekaj na wynik automatycznej oceny.

## Opis zadania
- Każda niepusta linia wejścia ma format `suite::case | outcome | duration_ms`. Zignoruj linie zaczynające się od `#` oraz te zawierające wyłącznie białe znaki.
- Obsługuj wyniki `pass`, `fail`, `skip` bez względu na wielkość liter. Dla innych wartości zwróć `ReportError::InvalidOutcome { raw }` z oryginalnym tekstem.
- Rozdziel sekcję `suite::case` na nazwę pakietu (`suite`) i nazwę testu (`case`). Brak separatora `::` traktuj jako `ReportError::InvalidFormat { line }`.
- Przed parsowaniem przytnij nadmiarowe spacje wokół każdej z sekcji (`suite::case`, `outcome`, `duration_ms`).
- Czas trwania (`duration_ms`) parsuj do `u64`. Błędną liczbę raportuj jako `ReportError::InvalidDuration { raw }`.
- Zaimplementuj `TestCase::from_line`, `parse_cases`, `summarize_by_suite`, `format_summary` oraz `collect_report`. Po odfiltrowaniu pustych i komentarzy, brak przypadków powinien kończyć się `ReportError::NoCases`.
- `summarize_by_suite` powinno agregować przypadki według `suite`, licząc łączną liczbę testów oraz rozbicie na `passed`, `failed`, `skipped`, a także sumaryczny czas `total_duration_ms`. Zwracaj dane uporządkowane alfabetycznie po nazwie pakietu.
- `format_summary` przygotowuje linie tekstowe w stylu `Suite Checkout: 4 przypadki (pass: 3, fail: 1, skip: 0) - łączny czas 875ms`. Wszystkie pakiety powinny korzystać z tej samej struktury opisu.
- `fmt::Display` dla `ReportError` powinien zwracać komunikaty: `Niepoprawny format linii: <linia>`, `Nieznany wynik testu: <wartość>`, `Niepoprawny czas trwania: <wartość>` oraz `Brak przypadków testowych`.
- Połącz wszystko w `collect_report`, która powinna korzystać z operatora `?` do propagowania błędów.

## Nowe pojęcia
- **`#[cfg(test)]` i funkcje oznaczone `#[test]`**  
  Moduł testowy pozwala w tym samym pliku co implementacja zapisać przypadki weryfikujące logikę. Każdy test jest zwykłą funkcją bez argumentów, a asercje korzystają z makr `assert_eq!`, `assert!` itp.

  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn parser_zwraca_blad_dla_zlego_formatu() {
          let err = TestCase::from_line("broken line").unwrap_err();
          assert!(matches!(err, ReportError::InvalidFormat { .. }));
      }
  }
  ```

- **Wczesne wychodzenie z funkcji przez operator `?`**  
  Dzięki `?` możesz natychmiast zwrócić błąd i zachować czytelność przepływu. To szczególnie przydatne, gdy kolejne kroki zależą od wyniku wcześniejszych operacji.

  ```rust
  pub fn collect_report<R: BufRead>(reader: R) -> Result<Vec<String>, ReportError> {
      let cases = parse_cases(reader)?;
      let stats = summarize_by_suite(&cases);
      Ok(format_summary(&stats))
  }
  ```

- **`str::split_once` do czytelnego rozbijania wpisów**  
  Funkcja zwraca tuple rozdzielonych fragmentów i pozwala prosto obsłużyć brak separatora.

  ```rust
  let (suite, case) = raw_suite
      .split_once("::")
      .ok_or_else(|| ReportError::InvalidFormat { line: raw_line.to_string() })?;
  ```

## Ekstra podpowiedzi
- Użyj `to_ascii_lowercase()` aby rozwiązać problem wielkości liter przy wynikach testu.
- `BTreeMap` lub `HashMap` z manualnym sortowaniem pomogą ustawić pakiety w stabilnej kolejności – wybierz rozwiązanie, które uważasz za czytelniejsze.
- Oddziel osobno parsowanie (`parse_cases`) od agregacji (`summarize_by_suite`), żeby łatwiej pisać i testować logikę.
- W modułowych testach pokryj zarówno poprawne ścieżki, jak i błędne formaty – to przyspieszy debugowanie.

## Uwaga
Nie modyfikuj plików ani interfejsów spoza miejsc oznaczonych `todo!()`. Zmiany poza zakresem zadania mogą skutkować niższą oceną.
