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
        if capacity == 0 {
            return Err(BufferError::ZeroCapacity);
        }
        let mut slots = Vec::<Option<T>>::with_capacity(capacity);
        for _ in 0..capacity {
            slots.push(None);
        }
        let head = 0;
        let len = 0;
        Ok(RingBuffer { slots, head, len })
    }

    /// Tworzy bufor z elementów iteratora, nadpisując najstarsze dane gdy brak miejsca.
    pub fn from_iter<I>(capacity: usize, source: I) -> Result<Self, BufferError>
    where
        I: IntoIterator<Item = T>,
    {
        let mut buf = RingBuffer::<T>::new(capacity)?;
        for el in source {
            buf.push(el);
        }
        Ok(buf)
    }

    /// Liczba przechowywanych elementów.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Pojemność bufora.
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    /// Sprawdza, czy bufor jest pusty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Sprawdza, czy bufor jest pełny.
    pub fn is_full(&self) -> bool {
        self.len == self.slots.len()
    }

    /// Dodaje element, zwraca nadpisany (najstarszy) element jeśli bufor był pełny.
    pub fn push(&mut self, value: T) -> Option<T> {
        if ! self.is_full() {
            self.len += 1;
        }
        let evicted = std::mem::replace(&mut self.slots[self.head], Some(value));
        self.head = (self.head + 1) % self.slots.len();
        evicted
    }

    /// Zwraca iterator po elementach od najstarszego do najnowszego.
    pub fn iter(&self) -> RingIter<'_, T> {
        self.into_iter()
    }

    /// Zwraca iterator po `count` ostatnich elementach (lub wszystkich, jeśli jest ich mniej).
    pub fn iter_recent(&self, count: usize) -> RingIter<'_, T> {
        let slots = self.slots.as_slice();
        let count = std::cmp::min(count, self.len());
        let offset = self.len() - count;

        let mut start = 0;
        if self.is_full() {
            start = self.head;
        }
        start += offset;

        let front_offset = 0;
        let back_offset = count;
        let remaining = count;
        let capacity = self.slots.len();

        RingIter {
            slots,
            start,
            front_offset,
            back_offset,
            remaining,
            capacity
        }
    }

    /// Zwraca posortowaną kopię elementów na podstawie przekazanego komparatora.
    pub fn into_sorted_vec_by<F>(&self, mut cmp: F) -> Vec<T>
    where
        T: Clone,
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        let mut vec = Vec::<T>::with_capacity(self.len());
        for el in self.slots.iter() {
            let el = el.as_ref();
            if ! el.is_none() {
                vec.push(el.unwrap().clone());
            }
        }
        vec.sort_by(cmp);
        vec
    }

    /// Wyszukuje ostatni element spełniający predykat.
    pub fn find_last<P>(&self, mut predicate: P) -> Option<&T>
    where
        P: FnMut(&T) -> bool,
    {
        let mut idx = self.head;
        loop {
            if idx == 0 {
                break;
            }
            idx -= 1;
            let el = &self.slots[idx];
            let el = el.as_ref();
            if predicate(el.unwrap()) {
                return el;
            }
        }
        let mut idx = self.slots.len() - 1;
        loop {
            let el = &self.slots[idx];
            let el = el.as_ref();
            if el.is_none() {
                return None;
            }
            if predicate(el.unwrap()) {
                return el;
            }
            if idx == self.head {
                break;
            }
            idx -= 1;
        }
        return None;
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
        if self.remaining == 0 {
            return None;
        }
        let idx = (self.start + self.front_offset) % self.capacity;
        self.front_offset += 1;
        self.remaining -= 1;
        self.slots[idx].as_ref()
    }
}

impl<'a, T> DoubleEndedIterator for RingIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.back_offset -= 1;
        self.remaining -= 1;
        let idx = (self.start + self.back_offset) % self.capacity;
        self.slots[idx].as_ref()
    }
}

impl<'a, T> IntoIterator for &'a RingBuffer<T> {
    type Item = &'a T;
    type IntoIter = RingIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let slots = self.slots.as_slice(); 

        let capacity = self.slots.len();
        let remaining = self.len();
        let front_offset = 0;
        let back_offset = self.len();

        let mut start = 0;
        if self.is_full() {
            start = self.head;
        }
        Self::IntoIter {
            slots,
            capacity,
            remaining,
            front_offset,
            back_offset,
            start
        }
    }
}
