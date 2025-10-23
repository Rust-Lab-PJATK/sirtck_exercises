use std::io::{self, BufRead, Write};

pub struct FastScanner<R> {
    reader: R,
    buf: Vec<u8>,
    pos: usize,
}

impl<R: BufRead> FastScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: Vec::new(),
            pos: 0,
        }
    }

    pub fn next_i64(&mut self) -> i64 {
        todo!("zaimplementuj szybkie odczytywanie i64");
    }

    pub fn next_usize(&mut self) -> usize {
        todo!("zaimplementuj szybkie odczytywanie usize");
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    RangeAdd { left: usize, right: usize, delta: i64 },
    RangeMin { left: usize, right: usize },
}

pub struct SegmentTree {
    size: usize,
    tree: Vec<i64>,
    lazy: Vec<i64>,
}

impl SegmentTree {
    pub fn from(values: &[i64]) -> Self {
        todo!("zbuduj drzewo segmentowe z wartosciami startowymi");
    }

    fn apply(&mut self, node: usize, delta: i64) {
        todo!("zastosuj leniwa aktualizacje dla wierzcholka");
    }

    fn push(&mut self, node: usize) {
        todo!("zepchnij zalegle aktualizacje do dzieci");
    }

    pub fn range_add(&mut self, left: usize, right: usize, delta: i64) {
        todo!("dodaj delta na przedziale [left, right)");
    }

    pub fn range_min(&mut self, left: usize, right: usize) -> i64 {
        todo!("zwroc minimum z przedzialu [left, right)");
    }
}

pub fn process_operations(n: usize, values: Vec<i64>, operations: Vec<Operation>) -> Vec<i64> {
    let _ = n;
    let _ = values;
    let _ = operations;
    todo!("obsluz sekwencje operacji i zwroc wyniki zapytan o minimum");
}

pub fn solve_from_reader<R: BufRead>(reader: R) -> Result<Vec<i64>, String> {
    let mut scanner = FastScanner::new(reader);
    let _ = &mut scanner;
    todo!("przetworz cale wejscie i zwroc wyniki zapytan");
}

pub fn solve_from_str(input: &str) -> Result<Vec<i64>, String> {
    let reader = std::io::Cursor::new(input.as_bytes());
    solve_from_reader(reader)
}

pub fn write_answers<W: Write>(answers: &[i64], writer: &mut W) -> io::Result<()> {
    let _ = answers;
    let _ = writer;
    todo!("wypisz wyniki, kazdy w osobnej linii");
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut handle = stdout.lock();
    let result = solve_from_reader(stdin.lock());

    match result {
        Ok(answers) => {
            if let Err(io_err) = write_answers(&answers, &mut handle) {
                eprintln!("I/O error: {}", io_err);
            }
        }
        Err(message) => {
            let _ = writeln!(handle, "{}", message);
        }
    }
}
