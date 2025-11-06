use std::io::{self, BufRead, Write};

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
        todo!("zwróć kolejną liczbę całkowitą jako usize z bufora wejściowego");
    }

    pub fn next_i64(&mut self) -> i64 {
        todo!("zwróć kolejną liczbę całkowitą jako i64 z bufora wejściowego");
    }
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub to: usize,
    pub rev: usize,
    pub capacity: i64,
    pub id: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct EdgeInput {
    pub from: usize,
    pub to: usize,
    pub capacity: i64,
    pub id: usize,
}

#[derive(Debug)]
pub struct Dinic {
    pub graph: Vec<Vec<Edge>>,
    pub level: Vec<i32>,
    pub iter: Vec<usize>,
}

impl Dinic {
    pub fn new(size: usize) -> Self {
        let _ = size;
        todo!("zainicjalizuj strukturę grafu z pustymi listami sąsiedztwa");
    }

    pub fn add_edge(&mut self, from: usize, to: usize, capacity: i64, id: usize) {
        let _ = (from, to, capacity, id);
        todo!("dodaj krawędź residualną i przeciwną z zachowaniem identyfikatora");
    }

    pub fn bfs(&mut self, source: usize, sink: usize) -> bool {
        let _ = (source, sink);
        todo!("zbuduj graf poziomów dla Dinica; zwróć true gdy sink osiągalny");
    }

    pub fn dfs(&mut self, v: usize, sink: usize, pushed: i64) -> i64 {
        let _ = (v, sink, pushed);
        todo!("spróbuj przepchnąć przepływ w dół grafu poziomów z optymalizacją current-arc");
    }
}

pub fn build_dinic(n: usize, edges: &[EdgeInput]) -> Dinic {
    let _ = (n, edges);
    todo!("utwórz strukturę Dinica i dodaj wszystkie krawędzie");
}

pub fn max_flow(dinic: &mut Dinic, source: usize, sink: usize) -> i64 {
    let _ = (dinic, source, sink);
    todo!("wykonaj pętlę Dinica (BFS + DFS) aż do wyczerpania blokujących przepływów");
}

pub fn reachable_in_residual(dinic: &Dinic, source: usize) -> Vec<bool> {
    let _ = (dinic, source);
    todo!("wyznacz wierzchołki osiągalne z source po krawędziach o dodatniej pojemności residualnej");
}

pub fn extract_reachable_vertices(flags: &[bool]) -> Vec<usize> {
    let _ = flags;
    todo!("przekonwertuj maskę osiągalności na posortowaną listę wierzchołków 1-indeksowanych");
}

pub fn cut_edges(flags: &[bool], edges: &[EdgeInput]) -> Vec<usize> {
    let _ = (flags, edges);
    todo!("wybierz identyfikatory krawędzi wychodzących ze zbioru S do dopełnienia");
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowSummary {
    pub max_flow: i64,
    pub reachable: Vec<usize>,
    pub cut_edges: Vec<usize>,
}

pub fn compute_flow_summary(
    n: usize,
    edges: Vec<EdgeInput>,
    source: usize,
    sink: usize,
) -> FlowSummary {
    let _ = (n, &edges, source, sink);
    todo!("policz maksymalny przepływ, wyznacz zbiór osiągalnych oraz listę krawędzi przekroju");
}

pub fn solve_from_reader<R: BufRead>(reader: R) -> Result<FlowSummary, String> {
    let mut scanner = FastScanner::new(reader);
    let _ = &mut scanner;
    todo!("wczytaj dane wejściowe, przekształć indeksy na 0-indeksowane i policz wynik");
}

pub fn solve_from_str(input: &str) -> Result<FlowSummary, String> {
    let cursor = io::Cursor::new(input.as_bytes());
    solve_from_reader(cursor)
}

pub fn write_answer<W: Write>(summary: &FlowSummary, writer: &mut W) -> io::Result<()> {
    let _ = (summary, writer);
    todo!("zapisz wynik zgodnie z formatem: przepływ, lista S, lista krawędzi przekroju");
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut handle = stdout.lock();

    match solve_from_reader(stdin.lock()) {
        Ok(summary) => {
            if let Err(error) = write_answer(&summary, &mut handle) {
                eprintln!("I/O error: {}", error);
            }
        }
        Err(message) => {
            let _ = writeln!(handle, "{}", message);
        }
    }
}
