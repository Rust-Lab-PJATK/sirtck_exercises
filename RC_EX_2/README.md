# Rate Limit Layer (`RC_EX_2`)

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `RC_EX_2`.
2. Na swoim branchu zaimplementuj wskazane elementy w pliku `src/main.rs`, zastępując miejsca oznaczone `todo!()`.
3. Stwórz Pull Request do brancha głównego z prefiksem `RC_EX_2`.
4. Po akceptacji PR lub uruchomieniu automatu otrzymasz informację zwrotną.

## Instrukcja zadania
- Zaimplementuj warstwę `RateLimitLayer`, która owija dowolny `tower::Service` i wykorzystuje współdzielony `TokenBucket` do ograniczania liczby wywołań w jednostce czasu.
- `TokenBucketConfig` ma walidować konfigurację: `capacity > 0`, `refill_amount > 0`, `refill_interval > 0`, a także `refill_amount <= capacity`. Nieprawidłowa konfiguracja powinna zwrócić szczegółowy `TokenBucketError`.
- `TokenBucket::acquire` zwraca `PermitFuture`, przyszłość oczekującą aż pojawi się wystarczająca liczba tokenów. Implementacja musi być bezpieczna w obecności wielu równoległych wywołań i nie może blokować w sekcji krytycznej (czekanie odbywa się poza blokadą).
- `Permit` pełni rolę strażnika: dopóki istnieje, tokeny są zarezerwowane. W momencie `Drop` oddaje niewykorzystane tokeny (co pozwala na obsługę błędów po stronie serwisu).
- `RateLimitService` powinien implementować `tower::Service` i propagować wynik z owiniętego serwisu. `poll_ready` deleguje do wewnętrznego serwisu, `call` pobiera pozwolenie (domyślnie 1 token) i dopiero wtedy przekazuje żądanie dalej. Błędy z limitera opakuj w `RateLimitError::RateLimit`, a błędy serwisu w `RateLimitError::Inner`.
- Zapewnij, że wiele konkurencyjnych żądań jest obsługiwanych w kolejności przybycia (FIFO) – kolejne zadania budzą się w takiej samej kolejności, w jakiej oczekują na tokeny.
- Sugerowana implementacja przechowuje `tokio::time::Sleep` w stanie `PermitFuture`, dzięki czemu wiele oczekujących zadań nie uruchamia aktywnego pollingu.

## Wprowadzenie do nowych pojęć
- `tokio::time::pause`, `advance` i deterministyczne testy czasu: pozwalają na symulowanie upływu czasu bez faktycznego czekania.

  ```rust
  #[tokio::test(start_paused = true)]
  async fn deterministic_time() {
      tokio::time::pause();
      let start = tokio::time::Instant::now();
      let sleep = tokio::time::sleep(Duration::from_secs(1));
      tokio::pin!(sleep);
      assert!(tokio::task::yield_now().await.is_ready());
      tokio::time::advance(Duration::from_secs(1)).await;
      sleep.await;
      assert_eq!(tokio::time::Instant::now(), start + Duration::from_secs(1));
  }
  ```

- `tower::Service` i `Layer`: interfejsy do budowania middleware'ów.

  ```rust
  use tower::{Layer, Service};

  #[derive(Clone)]
  struct LoggingLayer;

  impl<S> Layer<S> for LoggingLayer {
      type Service = LoggingService<S>;
      fn layer(&self, inner: S) -> Self::Service {
          LoggingService { inner }
      }
  }

  #[derive(Clone)]
  struct LoggingService<S> {
      inner: S,
  }

  impl<S, Request> Service<Request> for LoggingService<S>
  where
      S: Service<Request>,
  {
      type Response = S::Response;
      type Error = S::Error;
      type Future = S::Future;

      fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
          self.inner.poll_ready(cx)
      }

      fn call(&mut self, req: Request) -> Self::Future {
          println!("request!");
          self.inner.call(req)
      }
  }
  ```

- `Pin` i praca z `tokio::time::Sleep`: `Sleep` jest `!Unpin`, dlatego przechowujemy go w `Pin<Box<Sleep>>` lub korzystamy z `pin-project-lite`, żeby uprościć obsługę.

  ```rust
  use pin_project_lite::pin_project;
  use std::future::Future;
  use std::pin::Pin;
  use std::task::{Context, Poll};
  use tokio::time::{self, Sleep};

  pin_project! {
      struct DelayUntilReady {
          #[pin]
          sleeper: Sleep,
      }
  }

  impl Future for DelayUntilReady {
      type Output = ();

      fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
          let mut sleeper = self.project().sleeper;
          sleeper.poll(cx)
      }
  }
  ```

- `tokio::sync::Mutex` w kodzie asynchronicznym: pamiętaj o separacji sekcji krytycznej (lock szybko zwolniony przed `await`).

  ```rust
  use tokio::sync::Mutex;

  struct Shared {
      value: usize,
  }

  async fn mutate(shared: &Mutex<Shared>) {
      let mut guard = shared.lock().await;
      guard.value += 1;
  }
  ```

- `std::num::NonZeroUsize`: gwarantuje brak zera i upraszcza walidację.

  ```rust
  use std::num::NonZeroUsize;

  let tokens = NonZeroUsize::new(3).expect("wartość > 0");
  ```

## Ekstra podpowiedzi
- Trzymaj stan kubełka (`tokens`, `last_update`) w osobnej strukturze i odświeżaj go przy każdym wejściu do `acquire`.
- W `PermitFuture::poll` przełączaj się pomiędzy `Start` → `Sleeping`, przechowując `tokio::time::Sleep`. Gdy `Sleep` się zakończy, spróbuj ponownie pobrać tokeny bez blokowania.
- Do kolejności FIFO wystarczy trzymać pojedynczą `Mutex` i każdemu oczekującemu pozwolić zarejestrować własne `Sleep`. Jeżeli `Sleep` jest ponownie ustawiany, pamiętaj o anulowaniu poprzedniego (poprzez stworzenie nowego obiektu).
- W `Drop` dla `Permit` zadbaj, by oddać tylko niewykorzystane tokeny – jeśli użytkownik zwolni je ręcznie, strażnik może zostać oznaczony jako „zużyty”.

## Uwaga
Podczas rozwiązywania zadania modyfikuj tylko wskazane miejsca. Zmiany w pozostałych częściach projektu mogą spowodować odrzucenie rozwiązania.
