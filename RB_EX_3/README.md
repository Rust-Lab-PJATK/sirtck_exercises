# RB_EX_3 — Profilowanie przepływu telemetrii

Twoim zadaniem jest porównać dwa style przetwarzania danych telemetrycznych: rozbudowany pipeline iteratorów oraz klasyczne, imperatywne pętle. Projekt ma udostępniać prosty interfejs profilujący, aby można było podpiąć `cargo bench` i otrzymać czytelny raport czasów wykonania.

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `RB_EX_3`.
2. Na swoim branchu zaimplementuj brakujące fragmenty w katalogu `src/`.
3. Stwórz Pull Request do brancha głównego o prefixie `RB_EX_3`.
4. Poczekaj na wynik automatycznej oceny lub poproś administratora o ponowny bieg automatu, jeśli potrzebujesz kolejnej próby.

## Co masz zbudować
- Struktury opisujące pojedynczą próbkę telemetrii (`TelemetrySample`, `Status`) oraz końcowe zestawienie (`TelemetrySummary`).
- Dwie funkcje agregujące: `aggregate_with_iterators` wykorzystującą łańcuch wywołań iteratorów (`iter`, `filter`, `map`, `fold`, `collect`) oraz `aggregate_with_loops` opartą o jawne pętle i mutacje.
- Zbieranie statystyk obejmujących m.in. liczbę próbek, liczniki statusów, sumę bajtów payloadu, maksymalne opóźnienie oraz listę endpointów uznanych za wolne (>= `slow_threshold_ms`).
- Abstrakcję `Timer` z implementacją `InstantTimer`, która potrafi zmierzyć czas pojedynczego przebiegu strategii i zwrócić `Duration`.
- Funkcję `profile_strategies`, która przyjmuje liczbę powtórzeń (jako `NonZeroU32`), wywołuje każdą strategię tyle razy, korzysta z przekazanego timera i zwraca wektor raportów `ProfileReport` z łącznym i średnim czasem.
- Szablon benchmarku w `benches/pipelines.rs` (udostępniony w repozytorium) powinien działać po ukończeniu biblioteki i pozwalać uruchomić `cargo bench --bench pipelines`.

## Wymagania techniczne
- `TelemetrySummary::slow_endpoints` ma zawierać unikalne, posortowane rosnąco nazwy endpointów, które kiedykolwiek przekroczyły próg `slow_threshold_ms`.
- W pustym zbiorze danych `average_latency_ok` oraz `max_latency` powinny mieć wartość `None`.
- Funkcja profilująca musi korzystać z dostarczonego timera; w testach będzie podstawiany deterministyczny timer, dlatego nie polegaj na `Instant::now()` poza implementacją `InstantTimer`.
- `NonZeroU32` wymusza dodatnią liczbę iteracji, ale nadal obsłuż niepoprawny próg (`slow_threshold_ms == 0`) przez zwrócenie błędu.
- Kod powinien zwracać błędy domenowe (np. `ProfileError`) zamiast panik przy błędnych danych użytkownika.

## Nowe elementy na poziomie RB
- `std::time::Instant` i `Duration` pozwalają mierzyć czas fragmentów kodu w stabilnym Rust. Przykład:
  ```rust
  let start = Instant::now();
  expensive_operation();
  let elapsed = start.elapsed();
  println!("Operacja trwała {:?}", elapsed);
  ```
- `std::num::NonZeroU32` gwarantuje, że liczba powtórzeń profilowania jest dodatnia i dzięki temu eliminuje warunki specjalne:
  ```rust
  fn iterations(raw: u32) -> Result<NonZeroU32, ProfileError> {
      NonZeroU32::new(raw).ok_or(ProfileError::ZeroIterations)
  }
  ```
- Składanie iteratorów (`iter().filter(...).map(...).fold(...)`) pozwala pisać deklaratywnie i bez mutacji:
  ```rust
  let total_bytes = samples
      .iter()
      .filter(|s| s.status == Status::Ok)
      .map(|s| s.payload_bytes as u64)
      .fold(0u64, |acc, value| acc + value);
  ```
- `HashMap::entry` upraszcza inkrementację liczników:
  ```rust
  *summary.status_counts.entry(sample.status).or_insert(0) += 1;
  ```

## Ekstra podpowiedzi
- Przygotuj funkcję generującą dane testowe (np. z `rand` lub prostym seedem), aby szybko sprawdzić wyniki na większych wejściach.
- Zadbaj, aby `profile_strategies` nie alokowało zbędnych wektorów w każdej iteracji — klonuj dane wejściowe tylko wtedy, gdy to konieczne.
- Po ukończeniu implementacji uruchom `cargo test` oraz `cargo bench --bench pipelines`, by zobaczyć rzeczywisty raport czasów.
