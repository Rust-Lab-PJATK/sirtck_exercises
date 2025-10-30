use std::io::{self, BufRead, Write};

pub const LOG: usize = 19;

pub struct FastScanner<R> {
    reader: R,
    buffer: Vec<u8>,
    position: usize,
}

impl<R: BufRead> FastScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
            position: 0,
        }
    }

    pub fn next_usize(&mut self) -> usize {
        todo!("zwróć kolejną liczbę usize z bufora wejściowego");
    }

    pub fn next_i64(&mut self) -> i64 {
        todo!("zwróć kolejną liczbę i64 z bufora wejściowego");
    }

    pub fn next_string(&mut self) -> String {
        todo!("zwróć kolejne słowo (ciąg znaków bez białych znaków)");
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub u: usize,
    pub v: usize,
    pub weight: i64,
}

#[derive(Debug, Clone)]
pub enum Query {
    Dist { u: usize, v: usize },
    MinEdge { u: usize, v: usize },
    Kth { u: usize, v: usize, k: usize },
}

pub struct TreeSolver {
    n: usize,
    depth: Vec<usize>,
    up: Vec<[usize; LOG]>,
    min_edge: Vec<[i64; LOG]>,
    dist: Vec<i64>,
}

impl TreeSolver {
    pub fn new(n: usize, edges: &[Edge]) -> Self {
        let _ = n;
        let _ = edges;
        todo!("zainicjalizuj strukturę binary lifting na podstawie drzewa wejściowego");
    }

    pub fn lca(&self, u: usize, v: usize) -> usize {
        let _ = (u, v);
        todo!("wyznacz najniższego wspólnego przodka wierzchołków u i v");
    }

    pub fn dist(&self, u: usize, v: usize) -> i64 {
        let _ = (u, v);
        todo!("policz sumę wag na ścieżce pomiędzy u i v");
    }

    pub fn min_edge(&self, u: usize, v: usize) -> i64 {
        let _ = (u, v);
        todo!("znajdź najmniejszą wagę krawędzi na ścieżce pomiędzy u i v");
    }

    pub fn kth_on_path(&self, u: usize, v: usize, k: usize) -> usize {
        let _ = (u, v, k);
        todo!("zwróć k-ty wierzchołek na ścieżce od u do v (1-indeksowany)");
    }
}

pub fn process_queries(solver: &TreeSolver, queries: Vec<Query>) -> Vec<String> {
    let _ = solver;
    let _ = &queries;
    todo!("odpowiedz na zapytania DIST/MIN/KTH i zwróć wyniki jako napisy");
}

pub fn solve_from_reader<R: BufRead>(reader: R) -> Result<Vec<String>, String> {
    let mut scanner = FastScanner::new(reader);
    let _ = &mut scanner;
    todo!("wczytaj dane, przygotuj solver i wypisz odpowiedzi na wszystkie zapytania");
}

pub fn solve_from_str(input: &str) -> Result<Vec<String>, String> {
    let cursor = io::Cursor::new(input.as_bytes());
    solve_from_reader(cursor)
}

pub fn write_answers<W: Write>(answers: &[String], writer: &mut W) -> io::Result<()> {
    let _ = answers;
    let _ = writer;
    todo!("wypisz wszystkie odpowiedzi, każdą w osobnej linii");
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut handle = stdout.lock();
    let result = solve_from_reader(stdin.lock());

    match result {
        Ok(answers) => {
            if let Err(error) = write_answers(&answers, &mut handle) {
                eprintln!("I/O error: {}", error);
            }
        }
        Err(message) => {
            let _ = writeln!(handle, "{}", message);
        }
    }
}
