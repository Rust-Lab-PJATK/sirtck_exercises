use std::collections::HashMap;
use std::fmt;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Default)]
pub struct LogSummary {
    pub total: usize,
    pub by_level: HashMap<LogLevel, usize>,
    pub errors_by_component: HashMap<String, usize>,
}

#[derive(Debug)]
pub enum PipelineError {
    WorkerCountZero,
    InvalidFormat(String),
    InvalidLevel(String),
    ChannelClosed,
    JoinFailure,
}

impl fmt::Display for PipelineError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Zaimplementuj przyjazny komunikat o błędzie dla PipelineError")
    }
}

impl std::error::Error for PipelineError {}

#[derive(Debug)]
pub struct LogAggregator {
    sender: Option<mpsc::Sender<Message>>,
    handles: Vec<JoinHandle<()>>,
    shared: Arc<Mutex<LogSummary>>,
}

#[derive(Debug)]
enum Message {
    Parsed(ParsedLog),
    Shutdown,
}

impl LogAggregator {
    pub fn new(worker_count: usize) -> Result<Self, PipelineError> {
        todo!("Zainicjalizuj kanał, stwórz wątki i przygotuj współdzielony stan")
    }

    pub fn submit_line(&self, line: String) -> Result<(), PipelineError> {
        todo!("Zwaliduj i sparsuj linię, a następnie wyślij ją do kanału jako zadanie")
    }

    pub fn finish(mut self) -> Result<LogSummary, PipelineError> {
        todo!("Wyślij sygnały zakończenia, dołącz wątki i zwróć zagregowane statystyki")
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsedLog {
    level: LogLevel,
    component: String,
    message: String,
}

fn parse_line(line: &str) -> Result<ParsedLog, PipelineError> {
    todo!("Rozbij linię formatu LEVEL;komponent;wiadomość i zwróć wynik")
}

fn spawn_worker(
    receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
    shared: Arc<Mutex<LogSummary>>,
) -> JoinHandle<()> {
    todo!("Pętla robocza pobierająca wiadomości z kanału i aktualizująca statystyki")
}
