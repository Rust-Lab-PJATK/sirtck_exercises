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
        //todo!("Zainicjalizuj wektor o długości `capacity` i ustaw wskaźniki");
        if capacity == 0 {
            return Err(BufferError::ZeroCapacity);
        }

        Ok(Self {
            slots: (0..capacity).map(|_| None).collect(),
            head: 0,
            len: 0,
        })
    }

    /// Tworzy bufor z elementów iteratora, nadpisując najstarsze dane gdy brak miejsca.
    pub fn from_iter<I>(capacity: usize, source: I) -> Result<Self, BufferError>
    where
        I: IntoIterator<Item = T>,
    {
        //todo!("Użyj `Self::new` i `push`, aby załadować elementy do bufora");
        let mut buffer = Self::new(capacity)?;

        for value in source {
            buffer.push(value);
        }

        Ok(buffer)
    }

    /// Liczba przechowywanych elementów.
    pub fn len(&self) -> usize {
        //todo!();
        self.len
    }

    /// Pojemność bufora.
    pub fn capacity(&self) -> usize {
        //todo!();
        self.slots.len()
    }

    /// Sprawdza, czy bufor jest pusty.
    pub fn is_empty(&self) -> bool {
        //todo!();
        self.len == 0
    }

    /// Sprawdza, czy bufor jest pełny.
    pub fn is_full(&self) -> bool {
        //todo!();
        self.len == self.capacity()
    }

    /// Dodaje element, zwraca nadpisany (najstarszy) element jeśli bufor był pełny.
    pub fn push(&mut self, value: T) -> Option<T> {
        //todo!("Obsłuż aktualizację `head` oraz `len`");
        let capacity = self.capacity();

        if self.is_full() {
            let old = self.slots[self.head].replace(value);

            self.head = (self.head + 1) % capacity;

            old
        } else {
            let index = (self.head + self.len) % capacity;

            self.slots[index] = Some(value);
            self.len += 1;

            None
        }
    }

    /// Zwraca iterator po elementach od najstarszego do najnowszego.
    pub fn iter(&self) -> RingIter<'_, T> {
        //todo!("Zbuduj iterator bazując na polach struktury");
        RingIter {
            slots: &self.slots,
            capacity: self.capacity(),
            start: self.head,
            remaining: self.len,
            front_offset: 0,
            back_offset: 0,
        }
    }

    /// Zwraca iterator po `count` ostatnich elementach (lub wszystkich, jeśli jest ich mniej).
    pub fn iter_recent(&self, count: usize) -> RingIter<'_, T> {
       // todo!("Wykorzystaj `iter` i ogranicz liczbę elementów w iteratorze");
               let count = count.min(self.len);

        RingIter {
            slots: &self.slots,
            capacity: self.capacity(),
            start: (self.head + self.len - count) % self.capacity(),
            remaining: count,
            front_offset: 0,
            back_offset: 0,
        }
    }

    /// Zwraca posortowaną kopię elementów na podstawie przekazanego komparatora.
    pub fn into_sorted_vec_by<F>(&self, mut cmp: F) -> Vec<T>
    where
        T: Clone,
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        //todo!("Skopiuj dane do `Vec<T>` i posortuj z użyciem `sort_by`");
        let mut result: Vec<T> = self.iter().cloned().collect();

        result.sort_by(&mut cmp);

        result
    }

    /// Wyszukuje ostatni element spełniający predykat.
    pub fn find_last<P>(&self, mut predicate: P) -> Option<&T>
    where
        P: FnMut(&T) -> bool,
    {
        //todo!("Przejdź od najnowszego do najstarszego i zwróć referencję");
         self.iter().rev().find(|value| predicate(value))
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
        //todo!("Zwróć kolejną referencję, aktualizując licznik przodu");
        if self.front_offset + self.back_offset >= self.remaining {
            return None;
        }

        let index = (self.start + self.front_offset) % self.capacity;
        self.front_offset += 1;

        self.slots[index].as_ref()
    }
}

impl<'a, T> DoubleEndedIterator for RingIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        //todo!("Obsłuż iterację od końca, niezależnie od `next`");
        if self.front_offset + self.back_offset >= self.remaining {
            return None;
        }

        let index = (self.start + self.remaining - 1 - self.back_offset)
            % self.capacity;

        self.back_offset += 1;

        self.slots[index].as_ref()
    }
}

impl<'a, T> IntoIterator for &'a RingBuffer<T> {
    type Item = &'a T;
    type IntoIter = RingIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        //todo!("Pozwól na iterację po referencji do bufora");
        self.iter()
    }
}
