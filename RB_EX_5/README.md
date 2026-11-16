# RB_EX_5 — Asynchroniczny most logów JSON

Twoim zadaniem jest zbudowanie biblioteki, która wystawia asynchroniczne API do kolejkowania paczek logów JSON, ale całą pracę blokującą (parsowanie, agregację) wykonuje wewnętrzna pula wątków. Most pomiędzy światem `async` a workerami ma być oparty o kanał `tokio::sync::mpsc` (strona asynchroniczna) oraz `blocking_recv` po stronie wątków. Każde zgłoszenie jest rozliczane indywidualnie przez kanał `tokio::sync::oneshot`, a metoda `shutdown` zamyka pipeline i zwraca raport zbiorczy.

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `RB_EX_5`.
2. Na swoim branchu zaimplementuj brakujące elementy w katalogu `src/`.
3. Stwórz Pull Request do brancha głównego o prefixie `RB_EX_5`.
4. Poczekaj na wynik automatycznej oceny albo poproś administratora o ponowne uruchomienie automatu, jeśli potrzebujesz kolejnej próby.

## Co masz zbudować
- Strukturę `BatchIngestor` zarządzającą mostem async→blokujące z metodami:
  - `new(BridgeConfig)` — konfiguracja kanałów, uruchomienie `worker_count` wątków i przechowanie uchwytów,
  - `submit_batch(&self, Vec<String>)` — asynchroniczne wysłanie paczki logów, oczekiwanie na raport i propagacja błędów kanału,
  - `shutdown(self)` — wysłanie sygnałów stopu, dołączenie wszystkich wątków (bez blokowania runtime'u) oraz zwrócenie `BridgeSummary`.
- Typy domenowe: `BridgeConfig`, `BridgeError`, `BatchReport`, `BridgeSummary`, `LogLevel`, `SlowCall` oraz pomocnicza funkcja `parse_level`.
- Mechanizm kolejkowania z limitem pojemności (`queue_capacity`) zapewniający backpressure — po wypełnieniu kolejki kolejne `submit_batch` ma czekać, dopóki któryś z workerów nie odbierze zadania.
- Przetwarzanie paczki w wątku: parsowanie każdego wpisu JSON, odrzucanie błędnych rekordów, zliczanie per poziom i per serwis, sumowanie `payload_bytes` oraz wykrywanie wolnych rekordów (`duration_ms >= slow_call_limit_ms`).
- Obsługę parametru `simulate_latency` w konfiguracji — jeżeli jest ustawiony, worker powinien `std::thread::sleep` o zadany czas po przetworzeniu paczki (symulacja blokującego I/O, potrzebna by testować kolejkę).
- Kontrolowane zamknięcie: wysłanie tylu komunikatów stopu ile jest workerów, zamknięcie wszystkich kanałów i zebranie końcowego raportu w `shutdown`.

## Wymagania techniczne
- `BatchIngestor` ma być klonowalny (`Clone`) — każdy klon korzysta z tego samego mostu, a `shutdown` unieważnia wszystkie uchwyty i kolejne `submit_batch` ma zwracać `BridgeError::Stopped`.
- `submit_batch` zwraca `BridgeError::ChannelClosed`, jeśli kanał został niespodziewanie zamknięty (np. worker panicował) oraz `BridgeError::Stopped`, jeśli pipeline został zamknięty wcześniej.
- Kanał po stronie async wykorzystuje `async_channel::bounded` lub `tokio::sync::mpsc::channel` z `blocking_recv`, a wątki nie mogą blokować runtime'u (żadnego `tokio::task::block_in_place` ani `std::thread::sleep` w zadaniach async).
- Raporty muszą być deterministyczne — `per_service` i `per_level` w `BatchReport` oraz `BridgeSummary` przechowuj w `BTreeMap`, a `slow_calls` sortuj malejąco po `duration_ms`, następnie rosnąco po `service`.
- `shutdown` nie blokuje wprost wątku async: dołączanie wątków wykonaj wewnątrz `tokio::task::spawn_blocking` lub równoważnej konstrukcji.
- Każdy worker aktualizuje wspólny stan (statystyki) w sposób bezpieczny dla wątków; unikaj `unwrap` przy obsłudze kanałów.

## Nowe elementy na poziomie RB
- `tokio::sync::mpsc` można wykorzystać jako pomost async→sync — strona `async` wywołuje `send().await`, a worker odbiera przez `blocking_recv()`:
  ```rust
  let (tx, rx) = tokio::sync::mpsc::channel(16);
  std::thread::spawn(move || {
      while let Some(job) = rx.blocking_recv() {
          handle(job);
      }
  });
  ```
- Kanał `tokio::sync::oneshot` pozwala przywrócić wynik z wątku do świata async bez blokowania:
  ```rust
  let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
  tx.send(Job { payload, resp_tx }).await?;
  let report = resp_rx.await.map_err(|_| BridgeError::ChannelClosed)?;
  ```
- `async-channel` udostępnia wielokrotnych konsumentów z metodą `recv_blocking`, co upraszcza dzielenie kolejki między workerów.
- `tokio::task::spawn_blocking` izoluje kosztowne operacje (`join` na wątkach) i nie blokuje reaktywnego executora:
  ```rust
  tokio::task::spawn_blocking(move || {
      for handle in handles {
          handle.join().map_err(|_| BridgeError::WorkerExited)?;
      }
      Ok(summary)
  })
  .await
  .unwrap()
  ```

## Ekstra podpowiedzi
- Warto przechowywać stan zbiorczy w `Arc<Mutex<BridgeSummary>>` i aktualizować go po każdym sukcesie; `MutexGuard` trzymaj krótko.
- Do parsowania JSON użyj `serde_json::from_str`; błędne wpisy traktuj jako odrzucone, bez przerywania całej paczki.
- Przy aktualizacji map używaj `BTreeMap::entry`:
  ```rust
  *summary.per_service.entry(service).or_insert(0) += 1;
  ```
- `simulate_latency` wykorzystasz w testach, aby zasymulować długie operacje dyskowe — pamiętaj, że dotyczy to tylko strony workerów.
- Po zamknięciu mostu pamiętaj o opróżnieniu kolejki (odbierz wszystkie zadania zanim wyślesz sygnały stopu), aby uniknąć utraty pracy.
