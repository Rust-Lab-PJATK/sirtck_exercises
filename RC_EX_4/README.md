# Strumieniowy Game of Life na WebSocketach (`RC_EX_4`)

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `RC_EX_4`.
2. Na swoim branchu uzupełnij pliki `src/lib.rs` oraz powiązane moduły, zastępując `todo!()` i wykorzystując podane struktury.
3. Stwórz Pull Request do brancha głównego z prefiksem `RC_EX_4`.
4. Po akceptacji PR lub ponownym uruchomieniu automatu otrzymasz informację zwrotną.

## Instrukcja zadania
- Zaimplementuj `LifeGrid`, który przelicza klasyczne reguły Conwaya, trzyma w wewnętrznym wektorze stan planszy i potrafi zwrócić ASCII (`.`/`#`) reprezentację każdego wiersza.
- Przygotuj `LifeServerConfig` walidujący interwał odświeżania (`tick_interval`) oraz rozmiar kolejki klientów (`client_buffer > 0`).
- `LifeServer::spawn` ma uruchomić zadanie tokio wykorzystujące `tokio::time::interval` (testy używają `time::pause/advance`) i natychmiast wysłać generację `0` do wszystkich subskrybentów.
- Każdy klient otrzymuje kolejkę MPSC o ograniczonej długości; przepełnienie oznacza usunięcie klienta. `LifeServer::subscribe` zwraca `LifeSubscription`, który implementuje `Stream<Item = LifeFrame>` i posiada metodę `next()`.
- `LifeFrame` przechowuje metadane (`generation`, `width`, `height`) oraz przypięty (`Pin<Box<str>>`) JSON z polami `generation`, `width`, `height`, `rows` (ASCII). Metoda `to_message()` buduje `tungstenite::Message::Text`.
- Funkcja `LifeServer::serve_into` pobiera dowolny `Sink<Message>` (np. opakowanie WebSocket) i przekazuje ramki, domykając połączenie gdy klient nie odbiera wiadomości.
- `LifeServerHandle::shutdown` powinien sygnalizować zakończenie pętli symulacji, dołączyć do zadania i zmapować błędy na `LifeServerError`.

## Wprowadzenie do nowych pojęć
- `tokio::time::interval` współpracuje z `tokio::time::pause/advance`, co pozwala deterministycznie testować upływ czasu:

  ```rust
  tokio::time::pause();
  let mut ticker = tokio::time::interval(Duration::from_millis(100));
  ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
  ticker.tick().await; // natychmiastowa pierwsza klatka
  tokio::time::advance(Duration::from_millis(100)).await;
  ticker.tick().await; // kolejna generacja
  ```

- `pin_project` i `Pin<Box<str>>` gwarantują, że bufor JSON nie zostanie przeniesiony podczas asynchronicznego wysyłania:

  ```rust
  #[pin_project]
  pub struct LifeSubscription {
      #[pin]
      receiver: mpsc::Receiver<LifeFrame>,
  }

  let json = serde_json::to_string(&payload)?;
  let pinned = Box::pin(json.into_boxed_str());
  ```

- Integracja z `Sink<Message>` z `futures` pozwala obsłużyć różne transporty WebSocket:

  ```rust
  pub async fn forward<S>(mut stream: LifeSubscription, mut sink: S) -> Result<(), LifeServerError>
  where
      S: Sink<Message> + Unpin,
      S::Error: std::error::Error + Send + Sync + 'static,
  {
      while let Some(frame) = stream.next().await {
          sink.send(frame.to_message()).await.map_err(|err| LifeServerError::Sink(err.to_string()))?;
      }
      Ok(())
  }
  ```

- JSON ramki powinien wyglądać następująco:

  ```json
  {
    "generation": 3,
    "width": 5,
    "height": 5,
    "rows": [
      "..#..",
      "...#.",
      ".###.",
      ".....",
      "....."
    ]
  }
  ```

## Ekstra podpowiedzi
- Rozważ przechowywanie najnowszej ramki w `Arc` i klonowanie przy publikacji — ułatwia to obsługę spóźnionych klientów i zapewnia współdzielenie przypiętego bufora.
- Wykorzystaj `tokio::sync::watch` lub `broadcast` do powiadamiania o zakończeniu pracy i `tokio::select!` by łączyć sygnały z generacjami.
- Do kontroli przepełnienia kolejki przydaje się `mpsc::Sender::try_send`; w przypadku błędu `Full` usuń kanał klienta.
- Backpressure warto mierzyć licznikiem generacji: jeśli klient zalega, możesz logować zdarzenia lub zwracać własny wariant `LifeServerError`.
- Przy testach manualnych uruchom `cargo test -- --nocapture` w `tasks/T_RC_EX_4`, aby obserwować logi symulacji.

## Uwaga
Podczas rozwiązywania zadania modyfikuj tylko wskazane miejsca. Zmiany w pozostałych częściach projektu mogą spowodować odrzucenie rozwiązania.
