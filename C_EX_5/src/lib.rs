#![allow(dead_code)]

/// Błąd tworzenia siatki automatu.
#[derive(Debug)]
pub enum GridBuildError {
    ZeroWidth,
    EmptyCells,
    IncompatibleDimensions { width: usize, cells: usize },
}

/// Dwuwymiarowa siatka przechowywana w spłaszczonej postaci.
#[derive(Debug, Clone)]
pub struct Grid<Cell> {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl<Cell> Grid<Cell> {
    /// Tworzy nową siatkę na podstawie szerokości i spłaszczonych danych.
    pub fn new(width: usize, cells: Vec<Cell>) -> Result<Self, GridBuildError> {
        todo!("Sprawdź poprawność danych i wylicz wysokość siatki");
    }

    /// Zwraca szerokość siatki.
    pub fn width(&self) -> usize {
        todo!()
    }

    /// Zwraca wysokość siatki.
    pub fn height(&self) -> usize {
        todo!()
    }

    /// Zwraca wszystkie komórki w kolejności wiersz-po-wierszu.
    pub fn cells(&self) -> &[Cell] {
        todo!()
    }

    /// Zwraca pożyczony wiersz o indeksie `y`.
    pub fn row(&self, y: usize) -> Option<&[Cell]> {
        todo!("Zwróć `None`, jeśli `y` przekracza wysokość siatki")
    }

    /// Zwraca sąsiedztwo 3x3 wokół przekazanej pozycji.
    pub fn neighborhood(&self, position: Position) -> Neighborhood<'_, Cell> {
        todo!("Zbierz północny, centralny i południowy segment wiersza")
    }

    /// Podmienia spłaszczone komórki na nowe, weryfikując rozmiar bufora.
    pub fn replace_cells(&mut self, cells: Vec<Cell>) -> Result<(), GridBuildError> {
        todo!("Zachowaj szerokość i wysokość, sprawdź liczbę elementów")
    }
}

/// Spójny fragment wiersza zaczynający się od indeksu `offset`.
#[derive(Debug, Clone, Copy)]
pub struct RowSegment<'a, Cell> {
    offset: usize,
    cells: &'a [Cell],
}

impl<'a, Cell> RowSegment<'a, Cell> {
    pub fn new(offset: usize, cells: &'a [Cell]) -> Self {
        Self { offset, cells }
    }

    pub fn offset(self) -> usize {
        todo!()
    }

    pub fn cells(self) -> &'a [Cell] {
        todo!()
    }

    pub fn len(self) -> usize {
        todo!()
    }

    pub fn contains(self, x: usize) -> bool {
        todo!()
    }

    pub fn at(self, x: usize) -> Option<&'a Cell> {
        todo!()
    }

    pub fn relative(self, focus_x: usize, dx: isize) -> Option<&'a Cell> {
        todo!("Skonwertuj `dx` na indeks względem `focus_x` i wywołaj `at`")
    }
}

/// Pozycja komórki w siatce.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// Widok na sąsiedztwo komórki, przechowujący pożyczone segmenty wierszy.
#[derive(Debug, Clone, Copy)]
pub struct Neighborhood<'a, Cell> {
    north: Option<RowSegment<'a, Cell>>,
    center: RowSegment<'a, Cell>,
    south: Option<RowSegment<'a, Cell>>,
    focus_x: usize,
}

impl<'a, Cell> Neighborhood<'a, Cell> {
    pub fn new(
        north: Option<RowSegment<'a, Cell>>,
        center: RowSegment<'a, Cell>,
        south: Option<RowSegment<'a, Cell>>,
        focus_x: usize,
    ) -> Self {
        Self {
            north,
            center,
            south,
            focus_x,
        }
    }

    pub fn focus_x(&self) -> usize {
        todo!()
    }

    pub fn center(&self) -> RowSegment<'a, Cell> {
        todo!()
    }

    pub fn north(&self) -> Option<RowSegment<'a, Cell>> {
        todo!()
    }

    pub fn south(&self) -> Option<RowSegment<'a, Cell>> {
        todo!()
    }

    pub fn focus(&self) -> &'a Cell {
        todo!("Zwróć referencję do centralnej komórki")
    }

    pub fn get(&self, dx: isize, dy: isize) -> Option<&'a Cell> {
        todo!("Zwróć sąsiada względem przesunięcia (dx, dy)")
    }
}

/// Reguła automatu komórkowego operująca na pożyczonym sąsiedztwie.
pub trait Rule<Cell> {
    type Error;

    fn apply<'a>(
        &mut self,
        position: Position,
        view: Neighborhood<'a, Cell>,
    ) -> Result<Cell, Self::Error>
    where
        Cell: 'a;
}

/// Silnik automatu wykonujący kolejne kroki symulacji.
pub struct Automaton<Cell, R> {
    grid: Grid<Cell>,
    rule: R,
}

impl<Cell, R> Automaton<Cell, R> {
    pub fn new(grid: Grid<Cell>, rule: R) -> Self {
        todo!("Zapisz przekazane argumenty w strukturze")
    }

    pub fn grid(&self) -> &Grid<Cell> {
        todo!()
    }
}

impl<Cell, R> Automaton<Cell, R>
where
    Cell: Clone,
    R: Rule<Cell>,
{
    pub fn step(&mut self) -> Result<(), R::Error> {
        todo!("Przejdź po wszystkich komórkach i zastosuj regułę")
    }

    pub fn step_many(&mut self, steps: usize) -> Result<(), R::Error> {
        todo!("Wykorzystaj `step`, aby wielokrotnie zaktualizować siatkę")
    }
}

impl<Cell, F, Error> Rule<Cell> for F
where
    F: for<'a> FnMut(Position, Neighborhood<'a, Cell>) -> Result<Cell, Error>,
{
    type Error = Error;

    fn apply<'a>(
        &mut self,
        position: Position,
        view: Neighborhood<'a, Cell>,
    ) -> Result<Cell, Self::Error>
    where
        Cell: 'a,
    {
        todo!("Wywołaj closurę, przekazując dalej pozycję i sąsiedztwo")
    }
}
