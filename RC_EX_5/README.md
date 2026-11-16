# Maszyna stanów handshake (`RC_EX_5`)

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `RC_EX_5`.
2. Na swoim branchu uzupełnij pliki `src/lib.rs` oraz ewentualne moduły pomocnicze, zastępując `todo!()` i wprowadzając wymaganą logikę.
3. Stwórz Pull Request do brancha głównego z prefiksem `RC_EX_5`.
4. Po akceptacji PR lub ponownym uruchomieniu automatu otrzymasz informację zwrotną.

## Instrukcja zadania
- Zaimplementuj `HandshakeMachine<B>`, która przechowuje backend `B: HandshakeBackend`, natychmiast rozpoczyna krok `resolve` i implementuje `Future<Output = Result<HandshakeOutcome<...>, HandshakeError>>`.
- Przechowuj stan w enumie oznaczonym `#[pin_project(project = ..., project_replace = ...)]`, tak aby w każdej chwili aktywne było wyłącznie jedno future (`resolve`, `connect` albo `authenticate`). Po zakończeniu kroku zamieniaj enum na kolejną fazę bez utraty przypięcia.
- Udostępnij `HandshakeMachine::phase(&self) -> HandshakePhase`, który zwraca bieżącą fazę (`Resolving`, `Connecting`, `Authenticating`, `Finished`). Dokładnie pilnuj inwariantu: faza zmienia się dopiero po ustawieniu następnego future.
- `StepFuture<T>` zawiera przypięte future i `CancellationHandle`; wykorzystaj je przy przejściach, aby backend był informowany o anulowaniu w razie `Drop`. Nie zapomnij o wywołaniu uchwytu w destruktorze maszyny, jeżeli handshake nie został ukończony.
- W przypadku sukcesu zwróć `HandshakeOutcome` z artefaktami `resolved`, `connection`, `session`. Błędy propaguj bez zmian. Jeśli błąd wystąpi w `resolve`, nie wywołuj już `connect`; analogicznie przy `connect` nie uruchamiaj `authenticate`.
- Zapewnij, że maszyna jest `Send` i może być bezpiecznie przypinana (`Unpin` nie jest wymagane). Wewnętrzne stany możesz dodatkowo dokumentować debug assertami – testy oczekują, że ukryte inwarianty są pilnowane.

## Wprowadzenie do nowych pojęć
- `project_replace` z `pin-project` pozwala bezpiecznie wymieniać warianty enumu przechowującego `!Unpin` future:

  ```rust
  use pin_project::pin_project;
  use std::future::Future;
  use std::pin::Pin;
  use std::task::{Context, Poll};

  #[pin_project(project = StateProj, project_replace = StateReplace)]
  enum State<F> {
      Step #[pin](F),
      Empty,
  }

  impl<F> State<F> {
      fn take_step(self: Pin<&mut Self>) -> Option<Pin<&mut F>> {
          match self.project() {
              StateProj::Step(fut) => Some(fut),
              StateProj::Empty => None,
          }
      }

      fn replace_with_empty(self: Pin<&mut Self>) -> Option<F> {
          match self.project_replace(State::Empty) {
              StateReplace::Step(fut) => Some(fut),
              StateReplace::Empty => None,
          }
      }
  }
  ```

- Ręczna implementacja `Future` wymaga pilnowania `Poll` i przejść stanów. Wzorzec z enumem i `loop` wewnątrz `poll` pozwala reagować na kolejne gotowe future:

  ```rust
  impl<F> Future for HandshakeMachine<F>
  where
      F: Future<Output = Result<(), HandshakeError>>,
  {
      type Output = F::Output;

      fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
          let mut this = self.project();
          loop {
              match this.state.as_mut().project() {
                  StateProj::Step(step) => match step.poll(cx) {
                      Poll::Pending => return Poll::Pending,
                      Poll::Ready(result) => { /* przejście do kolejnego wariantu */ }
                  },
                  StateProj::Empty => return Poll::Ready(Ok(())),
              }
          }
      }
  }
  ```

- `CancellationHandle` możesz budować na `Arc<dyn Fn(HandshakePhase)>`. Dzięki temu wiele clone'ów może informować backend o anulowaniu:

  ```rust
  use std::sync::Arc;

  #[derive(Clone)]
  struct CancellationHandle {
      inner: Arc<dyn Fn(HandshakePhase) + Send + Sync>,
  }

  impl CancellationHandle {
      fn new<F>(notify: F) -> Self
      where
          F: Fn(HandshakePhase) + Send + Sync + 'static,
      {
          Self { inner: Arc::new(notify) }
      }
  }
  ```

- Pamiętaj, że `todo!()` w `Drop` lub `poll` spowoduje panic podczas testów; walidacja wymaga pełnego, deterministycznego przepływu.

## Ekstra podpowiedzi
- Przechowuj artefakty (`resolved`, `connection`) w polach struktury zanim przejdziesz do kolejnego etapu – ułatwia to budowę końcowego `HandshakeOutcome`.
- W `Drop` możesz użyć `match self.phase` by dokładnie wiedzieć, którą fazę należy zgłosić przez `CancellationHandle`.
- `debug_assert!` w każdym przejściu stanu pomoże Ci szybko wykryć podwójne wywołanie `resolve` lub `connect`.
- Jeśli chcesz wystawiać dodatkową diagnostykę, rozważ `#[derive(Debug)]` dla własnych enumów stanów – testy sprawdzają głównie zachowanie, nie format.

## Uwaga
Podczas rozwiązywania zadania modyfikuj tylko wskazane miejsca. Zmiany w pozostałych częściach projektu mogą spowodować odrzucenie rozwiązania.

