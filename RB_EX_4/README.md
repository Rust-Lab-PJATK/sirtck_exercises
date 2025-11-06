# RB_EX_4 — Scope'owana analiza sensorów

Twoim zadaniem jest zbudowanie biblioteki, która analizuje pakiety pomiarów pochodzące z wielu sensorów, dzieląc pracę pomiędzy wątki uruchamiane w `std::thread::scope`. Każdy wątek oblicza lokalne statystyki, a następnie przekazuje wynik do wątku głównego przez kanał MPSC. Na końcu tworzony jest zbiorczy raport z wykrytymi anomaliami.

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `RB_EX_4`.
2. Na swoim branchu zaimplementuj brakujące elementy w katalogu `src/`.
3. Stwórz Pull Request do brancha głównego o prefixie `RB_EX_4`.
4. Poczekaj na wynik automatycznej oceny albo poproś administratora o ponowne uruchomienie automatu, jeśli potrzebujesz kolejnej próby.

## Co masz zbudować
- Struktury `SensorPacket<'a>`, `AnalyzerSettings`, `SensorStats`, `Anomaly<'a>` oraz `SensorSummary<'a>` opisujące wejście, konfigurację i wynik.
- Funkcję `analyze_sensors`, która przyjmuje `&[SensorPacket]`, liczbę wątków i konfigurację, uruchamia `worker_count` wątków wewnątrz `std::thread::scope`, rozdziela pomiary oraz odbiera częściowe raporty przez `std::sync::mpsc`.
- Mechanizm łączenia częściowych wyników w `BTreeMap<&str, SensorStats>` oraz detekcję anomalii:
  - średnia z przekroju sensorów powyżej `AnalyzerSettings::mean_limit` → `AlertKind::Average`,
  - maksimum próbki powyżej `AnalyzerSettings::peak_limit` → `AlertKind::Peak`,
  - całkowita liczba punktów mniejsza niż `AnalyzerSettings::min_points_for_average` → `AlertKind::InsufficientData`.
- Deterministyczne sortowanie `SensorSummary::anomalies` rosnąco po `sensor_id`, a następnie po rodzaju alertu.

## Wymagania techniczne
- `worker_count` musi być większe od zera; w przeciwnym razie funkcja ma zwrócić `AnalyzerError::WorkerCountZero`.
- Puste identyfikatory sensorów należy traktować jako błąd (`AnalyzerError::EmptySensorId`).
- Pakiet z pustą tablicą pomiarów powinien zakończyć analizę błędem (`AnalyzerError::NoSamples { sensor_id }`), bez uruchamiania wątków roboczych.
- Wątki robocze mają otrzymywać porcje danych jako referencje — unikaj kopiowania `String`/`Vec<f64>` tylko po to, by spełnić `'static`. To zadanie ma uczyć korzystania z `std::thread::scope`.
- `SensorSummary::stats` musi używać `BTreeMap`, aby wyniki były porządkowane deterministycznie. Średnia w `SensorStats` ma być `None`, jeśli liczba punktów jest mniejsza niż `min_points_for_average`.
- Po zakończeniu `analyze_sensors` wszystkie wątki powinny być dołączone (`join` wykonuje się automatycznie po wyjściu ze scope) i kanały zamknięte. Upewnij się, że odbiór z kanału nie blokuje się na zawsze.

## Nowe elementy na poziomie RB
- `std::thread::scope` pozwala uruchamiać wątki z żywotnością krótszą niż `'static`, co ułatwia przetwarzanie danych pożyczonych z otaczającego kontekstu:
  ```rust
  thread::scope(|scope| {
      for chunk in chunks {
          scope.spawn(move || {
              // chunk to slice pożyczone z otaczającego zakresu
              process(chunk);
          });
      }
  });
  ```
- Kanały `std::sync::mpsc` stanowią prosty sposób przekazywania wyników z wątków roboczych do wątku głównego. Wypuszczenie wszystkich nadawców (`Sender`) automatycznie kończy strumień:
  ```rust
  let (tx, rx) = mpsc::channel();
  scope.spawn({
      let tx = tx.clone();
      move || tx.send(worker_summary).expect("kanał działa")
  });
  drop(tx); // zamknij kanał po wysłaniu zadań
  while let Ok(summary) = rx.recv() {
      merge(summary);
  }
  ```
- `BTreeMap` zapewnia porządek według klucza i jest dobrym wyborem, gdy wynik powinien być deterministyczny. Możesz dzięki temu łatwo posortować raport anomalii bez dodatkowych kroków:
  ```rust
  let mut stats: BTreeMap<&str, SensorStats> = BTreeMap::new();
  stats.entry(sensor_id)
       .and_modify(|entry| entry.packets += 1)
       .or_insert(SensorStats::new());
  ```

## Ekstra podpowiedzi
- Podziel wejściowe pakiety na bloki zbliżone rozmiarem do `ceil(packets.len() / worker_count)` — unikniesz nierównomiernego obciążenia.
- Przechowuj w raportach cząstkowych sumę, liczbę próbek i maksimum; łączenie sprowadzi się do dodawania liczb oraz `f64::max`.
- Sortowanie anomalii jest prostsze, jeśli `AlertKind` zaimplementujesz jako `#[derive(Ord, PartialOrd)]` z kolejnością odpowiadającą wymaganiom.
- Przed uruchomieniem wątków zweryfikuj wejście (`sensor_id`, `readings`). Dzięki temu nie będziesz musiał obsługiwać wyjątków po stronie workerów.
- Połączone statystyki warto przeliczać w jednym miejscu, tak aby logika decydująca o anomaliach była spójna i łatwa do przetestowania.
