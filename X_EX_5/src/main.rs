use num_complex::Complex64;
use std::f64::consts::PI;
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

    fn refill_buffer(&mut self) -> io::Result<usize> {
        self.buffer.clear();
        self.position = 0;
        let bytes_read = self.reader.read_until(b'\n', &mut self.buffer)?;
        Ok(bytes_read)
    }

    fn peek_byte(&mut self) -> io::Result<Option<u8>> {
        if self.position >= self.buffer.len() {
            if self.refill_buffer()? == 0 {
                return Ok(None);
            }
        }
        Ok(self.buffer.get(self.position).copied())
    }

    fn skip_whitespace(&mut self) -> io::Result<()> {
        while let Some(byte) = self.peek_byte()? {
            if !byte.is_ascii_whitespace() {
                break;
            }
            self.position += 1;
        }
        Ok(())
    }

    pub fn next_usize(&mut self) -> usize {
        todo!("zwróć kolejną liczbę nieujemną jako usize z bufora wejściowego");
    }

    pub fn next_i64(&mut self) -> i64 {
        todo!("zwróć kolejną liczbę całkowitą jako i64 z bufora wejściowego");
    }
}

pub fn next_fft_len(len_a: usize, len_b: usize) -> usize {
    let mut size = 1usize;
    let target = len_a + len_b - 1;
    while size < target {
        size <<= 1;
    }
    size
}

pub fn bit_reverse_permute(values: &mut [Complex64]) {
    let _ = values;
    todo!("przestaw elementy w kolejności bit-reversal przed iteracyjną FFT");
}

pub fn prepare_twiddles(n: usize) -> Vec<Complex64> {
    let _ = n;
    todo!("zbuduj tablicę twiddle factorów exp(-2πik/n) do ponownego użycia");
}

pub fn fft_in_place(values: &mut [Complex64], twiddles: &[Complex64], inverse: bool) {
    let _ = (values, twiddles, inverse);
    todo!("zastosuj iteracyjną FFT z cache twiddle factorów; inverse steruje znakiem kąta");
}

pub fn convolution_i64(a: &[i64], b: &[i64]) -> Vec<i64> {
    let _ = (a, b);
    todo!("wykonaj splot dwóch sekwencji, korzystając z fft_in_place i ifft");
}

pub fn solve_from_reader<R: BufRead>(reader: R) -> Result<Vec<i64>, String> {
    let mut scanner = FastScanner::new(reader);
    let _ = &mut scanner;
    todo!("wczytaj n, m, q, wektory A i B oraz indeksy zapytań; zwróć wartości splotu dla żądanych pozycji");
}

pub fn solve_from_str(input: &str) -> Result<Vec<i64>, String> {
    let cursor = io::Cursor::new(input.as_bytes());
    solve_from_reader(cursor)
}

pub fn write_answers<W: Write>(answers: &[i64], writer: &mut W) -> io::Result<()> {
    let _ = (answers, writer);
    todo!("zapisz wyniki zapytań w osobnych liniach do podanego writer'a");
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut handle = stdout.lock();

    match solve_from_reader(stdin.lock()) {
        Ok(results) => {
            if let Err(error) = write_answers(&results, &mut handle) {
                eprintln!("I/O error: {}", error);
            }
        }
        Err(message) => {
            let _ = writeln!(handle, "{}", message);
        }
    }
}
