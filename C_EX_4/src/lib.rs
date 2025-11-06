use std::borrow::Cow;
use std::convert::Infallible;

/// Blok próbek przekazywany pomiędzy procesorami sygnału.
pub type Block<'a, Sample> = Cow<'a, [Sample]>;

/// Trait reprezentujący pojedynczy etap przetwarzania sygnału.
pub trait SignalProcessor {
    type Sample;
    type Error;

    fn process<'a>(
        &mut self,
        input: Block<'a, Self::Sample>,
    ) -> Result<Block<'a, Self::Sample>, Self::Error>;
}

/// Procesor skalujący każdą próbkę przez stały mnożnik.
#[derive(Clone, Copy, Debug)]
pub struct Gain<Sample> {
    factor: Sample,
}

impl<Sample> Gain<Sample> {
    pub fn new(factor: Sample) -> Self {
        todo!("Zapisz mnożnik w polu struktury");
    }

    pub fn factor(&self) -> Sample
    where
        Sample: Copy,
    {
        self.factor
    }
}

impl<Sample> SignalProcessor for Gain<Sample>
where
    Sample: Copy + std::ops::Mul<Output = Sample>,
{
    type Sample = Sample;
    type Error = Infallible;

    fn process<'a>(
        &mut self,
        input: Block<'a, Sample>,
    ) -> Result<Block<'a, Sample>, Self::Error> {
        todo!("Przemnóż wszystkie próbki przez `self.factor` wykorzystując `Cow`");
    }
}

/// Błąd łańcuchowego przetwarzania opakowujący błędy z poszczególnych etapów.
#[derive(Debug)]
pub enum ChainError<First, Second> {
    First(First),
    Second(Second),
}

/// Połączenie dwóch procesorów działających jeden po drugim.
pub struct Chain<P, Q> {
    first: P,
    second: Q,
}

impl<P, Q> SignalProcessor for Chain<P, Q>
where
    P: SignalProcessor,
    Q: SignalProcessor<Sample = P::Sample>,
{
    type Sample = P::Sample;
    type Error = ChainError<P::Error, Q::Error>;

    fn process<'a>(
        &mut self,
        input: Block<'a, Self::Sample>,
    ) -> Result<Block<'a, Self::Sample>, Self::Error> {
        todo!("Wywołaj `self.first`, a wynik przekaż do `self.second`");
    }
}

/// Procesor modyfikujący próbki za pomocą funkcji wyższego rzędu.
pub struct Map<P, F> {
    processor: P,
    mapper: F,
}

impl<P, F> SignalProcessor for Map<P, F>
where
    P: SignalProcessor,
    P::Sample: Clone,
    F: FnMut(&P::Sample) -> P::Sample,
{
    type Sample = P::Sample;
    type Error = P::Error;

    fn process<'a>(
        &mut self,
        input: Block<'a, Self::Sample>,
    ) -> Result<Block<'a, Self::Sample>, Self::Error> {
        todo!("Zastosuj mapper do wyniku wewnętrznego procesora");
    }
}

/// Procesor obserwujący blok próbek, nie modyfikując go.
pub struct Tap<P, F> {
    processor: P,
    observer: F,
}

impl<P, F> SignalProcessor for Tap<P, F>
where
    P: SignalProcessor,
    F: FnMut(&[P::Sample]),
{
    type Sample = P::Sample;
    type Error = P::Error;

    fn process<'a>(
        &mut self,
        input: Block<'a, Self::Sample>,
    ) -> Result<Block<'a, Self::Sample>, Self::Error> {
        todo!("Zaimplementuj obserwację bez modyfikacji sygnału");
    }
}

/// Zbiór rozszerzeń budujących złożone procesory przez kompozycję.
pub trait ProcessorExt: SignalProcessor + Sized {
    fn then<N>(self, next: N) -> Chain<Self, N>
    where
        N: SignalProcessor<Sample = Self::Sample>,
    {
        todo!("Zwróć `Chain` składający procesory");
    }

    fn map_samples<F>(self, mapper: F) -> Map<Self, F>
    where
        Self::Sample: Clone,
        F: FnMut(&Self::Sample) -> Self::Sample,
    {
        todo!("Zwróć `Map` stosujący mapper");
    }

    fn tap<F>(self, observer: F) -> Tap<Self, F>
    where
        F: FnMut(&[Self::Sample]),
    {
        todo!("Zwróć `Tap` pozwalający podejrzeć blok");
    }
}

impl<T> ProcessorExt for T where T: SignalProcessor {}

impl<F, Sample, Error> SignalProcessor for F
where
    F: for<'a> FnMut(Block<'a, Sample>) -> Result<Block<'a, Sample>, Error>,
{
    type Sample = Sample;
    type Error = Error;

    fn process<'a>(
        &mut self,
        input: Block<'a, Sample>,
    ) -> Result<Block<'a, Sample>, Error> {
        todo!("Deleguj wywołanie do closura `self`");
    }
}
