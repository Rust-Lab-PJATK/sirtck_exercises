use thiserror::Error;

/// Błąd konstrukcji lub użycia bufora.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum BufferError {
    /// Zwracany, gdy `capacity` przekazany do `RingBuffer::new` wynosi zero.
    #[error("capacity must be greater than zero")]
    ZeroCapacity,
}

/// Generyczny bufor cykliczny przechowujący ostatnie `capacity` elementów.
pub struct RingBuffer<T> {
    slots: Vec<Option<T>>,
    head: usize,
    len: usize,
}

impl<T> RingBuffer<T> {
    /// Tworzy bufor o zadanej pojemności. Zwraca błąd przy `capacity == 0`.
    pub fn new(capacity: usize) -> Result<Self, BufferError> {
        todo!("Zainicjalizuj wektor o długości `capacity` i ustaw wskaźniki");
    }

    /// Tworzy bufor z elementów iteratora, nadpisując najstarsze dane gdy brak miejsca.
    pub fn from_iter<I>(capacity: usize, source: I) -> Result<Self, BufferError>
    where
        I: IntoIterator<Item = T>,
    {
        todo!("Użyj `Self::new` i `push`, aby załadować elementy do bufora");
    }

    /// Liczba przechowywanych elementów.
    pub fn len(&self) -> usize {
        todo!();
    }

    /// Pojemność bufora.
    pub fn capacity(&self) -> usize {
        todo!();
    }

    /// Sprawdza, czy bufor jest pusty.
    pub fn is_empty(&self) -> bool {
        todo!();
    }

    /// Sprawdza, czy bufor jest pełny.
    pub fn is_full(&self) -> bool {
        todo!();
    }

    /// Dodaje element, zwraca nadpisany (najstarszy) element jeśli bufor był pełny.
    pub fn push(&mut self, value: T) -> Option<T> {
        todo!("Obsłuż aktualizację `head` oraz `len`");
    }

    /// Zwraca iterator po elementach od najstarszego do najnowszego.
    pub fn iter(&self) -> RingIter<'_, T> {
        todo!("Zbuduj iterator bazując na polach struktury");
    }

    /// Zwraca iterator po `count` ostatnich elementach (lub wszystkich, jeśli jest ich mniej).
    pub fn iter_recent(&self, count: usize) -> RingIter<'_, T> {
        todo!("Wykorzystaj `iter` i ogranicz liczbę elementów w iteratorze");
    }

    /// Zwraca posortowaną kopię elementów na podstawie przekazanego komparatora.
    pub fn into_sorted_vec_by<F>(&self, mut cmp: F) -> Vec<T>
    where
        T: Clone,
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        todo!("Skopiuj dane do `Vec<T>` i posortuj z użyciem `sort_by`");
    }

    /// Wyszukuje ostatni element spełniający predykat.
    pub fn find_last<P>(&self, mut predicate: P) -> Option<&T>
    where
        P: FnMut(&T) -> bool,
    {
        todo!("Przejdź od najnowszego do najstarszego i zwróć referencję");
    }
}

/// Iterator po referencjach do elementów bufora.
pub struct RingIter<'a, T> {
    slots: &'a [Option<T>],
    capacity: usize,
    start: usize,
    remaining: usize,
    front_offset: usize,
    back_offset: usize,
}

impl<'a, T> Iterator for RingIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Zwróć kolejną referencję, aktualizując licznik przodu");
    }
}

impl<'a, T> DoubleEndedIterator for RingIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        todo!("Obsłuż iterację od końca, niezależnie od `next`");
    }
}

impl<'a, T> IntoIterator for &'a RingBuffer<T> {
    type Item = &'a T;
    type IntoIter = RingIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!("Pozwól na iterację po referencji do bufora");
    }
}
