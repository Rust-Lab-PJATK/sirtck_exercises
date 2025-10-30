use std::collections::HashMap;
use std::fmt;
use std::num::NonZeroU32;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Ok,
    ClientError,
    ServerError,
}

#[derive(Debug, Clone)]
pub struct TelemetrySample {
    pub endpoint: String,
    pub latency_ms: u32,
    pub payload_bytes: u32,
    pub status: Status,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TelemetrySummary {
    pub total: usize,
    pub status_counts: HashMap<Status, usize>,
    pub total_payload_bytes: u64,
    pub max_latency: Option<u32>,
    pub average_latency_ok: Option<f64>,
    pub slow_endpoints: Vec<String>,
}

impl Default for TelemetrySummary {
    fn default() -> Self {
        Self {
            total: 0,
            status_counts: HashMap::new(),
            total_payload_bytes: 0,
            max_latency: None,
            average_latency_ok: None,
            slow_endpoints: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StrategyKind {
    Iterators,
    Loops,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProfileReport {
    pub strategy: StrategyKind,
    pub runs: NonZeroU32,
    pub total: Duration,
    pub average: Duration,
    pub summary: TelemetrySummary,
}

#[derive(Debug)]
pub enum ProfileError {
    ZeroIterations,
    ZeroThreshold,
    TimerUnavailable,
}

impl fmt::Display for ProfileError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Zaimplementuj przyjazny komunikat błędu dla ProfileError")
    }
}

impl std::error::Error for ProfileError {}

pub trait Timer {
    fn measure<F>(&mut self, strategy: StrategyKind, run: F) -> Result<Duration, ProfileError>
    where
        F: FnOnce();
}

#[derive(Default)]
pub struct InstantTimer;

impl Timer for InstantTimer {
    fn measure<F>(&mut self, strategy: StrategyKind, run: F) -> Result<Duration, ProfileError>
    where
        F: FnOnce(),
    {
        let _ = strategy; // zostanie użyte w implementacji
        todo!("Wykorzystaj Instant::now aby zmierzyć czas wykonania przekazanej strategii")
    }
}

pub fn aggregate_with_iterators(
    samples: &[TelemetrySample],
    slow_threshold_ms: u32,
) -> Result<TelemetrySummary, ProfileError> {
    let _ = (samples, slow_threshold_ms);
    todo!("Zbuduj zestawienie na bazie łańcucha iteratorów")
}

pub fn aggregate_with_loops(
    samples: &[TelemetrySample],
    slow_threshold_ms: u32,
) -> Result<TelemetrySummary, ProfileError> {
    let _ = (samples, slow_threshold_ms);
    todo!("Zbuduj zestawienie wykorzystując klasyczne pętle for")
}

pub fn profile_strategies<T: Timer>(
    samples: &[TelemetrySample],
    iterations: NonZeroU32,
    timer: &mut T,
    slow_threshold_ms: u32,
) -> Result<Vec<ProfileReport>, ProfileError> {
    let _ = (samples, iterations, timer, slow_threshold_ms);
    todo!("Wielokrotnie uruchom każdą strategię, zmierz czas i zwróć raporty")
}
