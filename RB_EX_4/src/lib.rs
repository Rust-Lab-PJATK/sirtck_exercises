use std::collections::BTreeMap;
use std::fmt;
use std::sync::mpsc;

/// Opis pojedynczego pakietu pomiarów dostarczonego przez sensor.
#[derive(Debug, Clone)]
pub struct SensorPacket<'a> {
    pub sensor_id: &'a str,
    pub readings: &'a [f64],
    pub window_ms: u64,
}

/// Konfiguracja progów wykrywania anomalii.
#[derive(Debug, Clone, Copy)]
pub struct AnalyzerSettings {
    pub mean_limit: f64,
    pub peak_limit: f64,
    pub min_points_for_average: usize,
}

/// Zestawienie statystyk dla pojedynczego sensora.
#[derive(Debug, Clone, PartialEq)]
pub struct SensorStats {
    pub packets: usize,
    pub points: usize,
    pub average: Option<f64>,
    pub max: f64,
}

impl SensorStats {
    pub fn new() -> Self {
        Self {
            packets: 0,
            points: 0,
            average: None,
            max: f64::NEG_INFINITY,
        }
    }
}

/// Klasyfikacja alertu wygenerowanego dla sensora.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertKind {
    Average,
    Peak,
    InsufficientData,
}

/// Opis anomalii wykrytej w wynikach.
#[derive(Debug, Clone, PartialEq)]
pub struct Anomaly<'a> {
    pub sensor_id: &'a str,
    pub kind: AlertKind,
    pub value: f64,
    pub limit: f64,
}

/// Raport końcowy zwracany przez analizę sensorów.
#[derive(Debug, Clone, PartialEq)]
pub struct SensorSummary<'a> {
    pub total_packets: usize,
    pub total_points: usize,
    pub stats: BTreeMap<&'a str, SensorStats>,
    pub anomalies: Vec<Anomaly<'a>>,
}

/// Cząstkowe statystyki wyznaczane przez wątki robocze.
#[derive(Debug, Clone, Default)]
pub struct PartialStats {
    pub packets: usize,
    pub points: usize,
    pub sum: f64,
    pub max: f64,
}

/// Raport z pojedynczego wątku, przekazywany kanałem MPSC.
#[derive(Debug)]
pub struct WorkerReport<'a> {
    pub totals: BTreeMap<&'a str, PartialStats>,
}

/// Błędy, które może zwrócić analiza.
#[derive(Debug)]
pub enum AnalyzerError {
    WorkerCountZero,
    EmptySensorId,
    NoSamples { sensor_id: String },
    ChannelClosed,
}

impl fmt::Display for AnalyzerError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Zaimplementuj czytelny opis błędu dla AnalyzerError")
    }
}

impl std::error::Error for AnalyzerError {}

/// Główna funkcja analizująca pomiary sensorów przy pomocy scope'owanych wątków.
pub fn analyze_sensors<'a>(
    packets: &'a [SensorPacket<'a>],
    worker_count: usize,
    settings: &AnalyzerSettings,
) -> Result<SensorSummary<'a>, AnalyzerError> {
    todo!("Zaimplementuj podział pracy na wątki, odbiór raportów i wykrywanie anomalii")
}

/// Tworzy raport cząstkowy dla kolekcji pakietów obsługiwanych przez pojedynczy wątek.
pub fn worker_report<'a>(packets: &'a [SensorPacket<'a>]) -> WorkerReport<'a> {
    todo!("Policz lokalne statystyki i zwróć je jako WorkerReport")
}

/// Łączy raporty cząstkowe w finalne zestawienie sensorów.
pub fn merge_reports<'a, I>(
    reports: I,
    settings: &AnalyzerSettings,
) -> Result<SensorSummary<'a>, AnalyzerError>
where
    I: IntoIterator<Item = WorkerReport<'a>>,
{
    todo!("Połącz raporty, wyznacz średnie i wybuduj listę anomalii")
}
