# RB_EX_2 — Wielowątkowy agregator logów

Twoim zadaniem jest zbudowanie niewielkiej biblioteki, która scala wpisy logów przetwarzane równolegle przez pulę wątków. Musisz zapewnić poprawną synchronizację danych oraz zamykanie pracy wątków.

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `RB_EX_2`.
2. Na swoim branchu zaimplementuj brakujące elementy w katalogu `src/`.
3. Stwórz Pull Request do brancha głównego o prefixie `RB_EX_2`.
4. Poczekaj na wynik automatycznej oceny albo poproś administratora o ponowne uruchomienie automatu, jeśli potrzebujesz kolejnej próby.

## Co masz zbudować
- Funkcję/strukturę udostępniającą API do wysyłania linii logów o formacie `LEVEL;komponent;wiadomość`.
- Kanał producent→pracownicy (`std::sync::mpsc`) rozdzielający linie pomiędzy wątki robocze.
- Mechanizm bezpiecznego współdzielenia zliczanych statystyk (`Arc<Mutex<...>>`), który zwraca na końcu:
  - łączną liczbę przetworzonych wpisów,
  - słownik liczby logów w rozbiciu na poziomy (`INFO`, `WARN`, `ERROR`),
  - słownik liczby błędów (`ERROR`) w rozbiciu na komponenty.
- Kontrolowane zamknięcie pipeline’u: przesłanie sygnałów zakończenia, zamknięcie kanału i `join` na wszystkich wątkach.
- Obsługę niepoprawnych danych (np. pusty poziom logu, poziom spoza listy, brak separatorów) z czytelnym błędem domenowym.

## Wymagania techniczne
- `worker_count` musi być większy od zera; traktuj 0 jako błąd wejściowy.
- Po wywołaniu metody kończącej nie wolno przyjmować nowych linii, a wszystkie wątki muszą zostać bezpiecznie dołączone.
- Brak wycieków danych: pamiętaj o zamknięciu kanału po zakończeniu wysyłania (drop nadawczej strony).
- Kod powinien przechodzić dostarczone testy jednostkowe i nie może polegać na `unwrap()` w ścieżkach błędów wywołanych przez użytkownika API.

## Nowe elementy na poziomie RB
- `Arc<T>` i `Mutex<T>` pozwalają współdzielić struktury danych pomiędzy wątkami. Najczęściej zapis wygląda tak:
  ```rust
  let shared = Arc::new(Mutex::new(LogSummary::default()));
  let worker_data = Arc::clone(&shared);
  thread::spawn(move || {
      let mut guard = worker_data.lock().expect("mutex poisoned");
      guard.total += 1;
  });
  ```
- Kanał `mpsc::channel()` daje parę `(Sender<T>, Receiver<T>)`. Po zamknięciu wszystkich `Sender` wątki nasłuchujące na `Receiver` otrzymają `Err(RecvError)` i mogą zakończyć pracę.
- Delikatne zatrzymanie wątków najłatwiej osiągnąć przez wysłanie specjalnej wiadomości typu `Message::Shutdown` tyle razy, ile mamy pracowników.
- Własny typ błędów można zbudować z pomocą `enum`, implementując `std::error::Error` i `Display`, np.:
  ```rust
  #[derive(Debug)]
  pub enum PipelineError {
      InvalidLevel(String),
      WorkerCountZero,
      ChannelDisconnect,
  }
  ```

## Ekstra podpowiedzi
- Użyj `BTreeMap` lub `HashMap` do przechowywania statystyk; ważne, aby aktualizacje były atomowe w sekcji chronionej przez `Mutex`.
- Wystandaryzuj reprezentację poziomu logu (`enum LogLevel`) i zmapuj ją z danych wejściowych za pomocą `match`.
- Pamiętaj, że `Sender::send` zwraca `Result`; traktuj błąd wysyłania jako okazję do przerwania pipeline’u z komunikatem o rozłączeniu.
- Zadbaj o testowalne API: metoda kończąca powinna zwracać gotowe statystyki, a nie wypisywać ich na standardowe wyjście.
