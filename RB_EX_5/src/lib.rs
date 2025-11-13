use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Duration;

use thiserror::Error;

/// Poziom logu, który należy sparsować z pola `level` w danych JSON.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Reprezentuje zgłoszenie powolnego rekordu (>= `BridgeConfig::slow_call_limit_ms`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlowCall {
    pub service: String,
    pub duration_ms: u64,
}

/// Raport zwracany po przetworzeniu pojedynczej paczki logów.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchReport {
    pub accepted: usize,
    pub rejected: usize,
    pub per_level: BTreeMap<LogLevel, usize>,
    pub per_service: BTreeMap<String, usize>,
    pub payload_bytes: u64,
    pub slow_calls: Vec<SlowCall>,
}

/// Raport zbiorczy zwracany przez `shutdown`, agregujący wszystkie paczki.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BridgeSummary {
    pub total_batches: usize,
    pub total_accepted: usize,
    pub total_rejected: usize,
    pub per_level: BTreeMap<LogLevel, usize>,
    pub per_service: BTreeMap<String, usize>,
    pub payload_bytes: u64,
}

/// Konfiguracja mostu pomiędzy światem `async` a blokującymi workerami.
#[derive(Debug, Clone)]
pub struct BridgeConfig {
    pub worker_count: NonZeroUsize,
    pub queue_capacity: NonZeroUsize,
    pub slow_call_limit_ms: u64,
    pub simulate_latency: Option<Duration>,
}

impl BridgeConfig {
    /// Pomaga w tworzeniu konfiguracji w testach i produkcji.
    pub fn new(
        worker_count: NonZeroUsize,
        queue_capacity: NonZeroUsize,
        slow_call_limit_ms: u64,
    ) -> Self {
        Self {
            worker_count,
            queue_capacity,
            slow_call_limit_ms,
            simulate_latency: None,
        }
    }
}

/// Błędy domenowe zwracane przez `BatchIngestor`.
#[derive(Debug, Error)]
pub enum BridgeError {
    #[error("bridge has already been shut down")]
    Stopped,
    #[error("internal channel closed unexpectedly")]
    ChannelClosed,
    #[error("worker thread panicked or could not be joined")]
    WorkerExited,
}

#[derive(Debug)]
struct Inner {
    // TODO: dodaj kanał async->sync, przechowaj konfigurację, uchwyty wątków i współdzielony stan.
    _marker: (),
}

/// Publiczny punkt wejścia — asynchroniczny most przekazujący paczki logów do workerów.
#[derive(Debug, Clone)]
pub struct BatchIngestor {
    inner: Arc<Inner>,
}

impl BatchIngestor {
    /// Inicjalizuje most, uruchamiając wątki workerów oraz przygotowując kanał z limitem pojemności.
    pub fn new(_config: BridgeConfig) -> Result<Self, BridgeError> {
        todo!("Skonfiguruj async-channel, uruchom wątki workerów i zapisz ich uchwyty w strukturze stanu");
    }

    /// Wysyła paczkę logów do kolejki i czeka na raport od blokującego workera.
    pub async fn submit_batch(&self, _batch: Vec<String>) -> Result<BatchReport, BridgeError> {
        todo!("Wyślij zadanie przez kanał mpsc, poczekaj na odpowiedź oneshot i obsłuż ewentualne błędy kanału");
    }

    /// Zatrzymuje pipeline – wysyła sygnały stopu, dołącza wątki i zwraca skumulowany raport.
    pub async fn shutdown(self) -> Result<BridgeSummary, BridgeError> {
        todo!("Wyślij do workerów sygnał zakończenia, połącz wątki (spawn_blocking) i zwróć zgromadzone statystyki");
    }
}

/// Pomocnicza funkcja mapująca nazwy poziomów na `LogLevel`.
pub fn parse_level(_raw: &str) -> Option<LogLevel> {
    todo!("Obsłuż poziomy DEBUG/INFO/WARN/ERROR ignorując wielkość liter");
}
