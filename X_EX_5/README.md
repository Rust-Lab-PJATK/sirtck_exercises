# FFT-owa korelacja sekwencji (`X_EX_5`)

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `X_EX_5`.
2. Na swoim branchu wypełnij wszystkie miejsca oznaczone `todo!()` w pliku `src/main.rs`. Nie dodawaj nowych plików.
3. Otwórz Pull Request do brancha głównego z prefiksem `X_EX_5`.
4. Po akceptacji lub zakończeniu automatycznej oceny otrzymasz informację zwrotną.

## Instrukcja zadania
Dane są dwie sekwencje liczb całkowitych `A` i `B`. Musisz obliczyć ich dyskretny splot
`C[k] = Σ_{i + j = k} A[i] * B[j]` dla wszystkich `k` od `0` do `n + m - 2`, a następnie odpowiedzieć na `q` zapytań o wartości `C[t]`.

- `1 ≤ n, m ≤ 2 * 10^5`
- `1 ≤ q ≤ 2 * 10^5`
- `|A[i]|, |B[j]| ≤ 10^6`
- Gwarantujemy, że wszystkie wyniki mieszczą się w zakresie `i64`.

### Format wejścia
```
n m q
A0 A1 ... A{n-1}
B0 B1 ... B{m-1}
t1
...
tq
```
Indeksy `t` są liczbami całkowitymi spełniającymi `0 ≤ t < n + m - 1`. Wejście może zawierać dowolną liczbę białych znaków pomiędzy liczbami.

### Format wyjścia
`q` linii, każda z pojedynczą liczbą całkowitą `C[ti]`.

### Przykład
Wejście:
```
4 3 4
1 0 -1 2
2 3 4
0
1
4
5
```
Wyjście:
```
2
3
5
8
```
Komentarz: Splot ma postać `[2, 3, 2, 7, 5, 8]`, więc dla zapytań `t = 0, 1, 4, 5` otrzymujemy odpowiednio `2, 3, 5, 8`.

## Wprowadzenie do nowych pojęć
- **Iteracyjna transformata Cooley–Tukey i permutacja bit-reversal** — zamiast rekurencji użyjemy kolejnych długości bloczków `len = 2, 4, 8, ...`. Na początku należy przestawić wektor w kolejności bit-reversal, aby późniejsze „motylki” scalały już sąsiadujące elementy:
  ```rust
  pub fn bit_reverse_permute(values: &mut [Complex64]) {
      let n = values.len();
      let mut j = 0usize;
      for i in 1..n {
          let mut bit = n >> 1;
          while j & bit != 0 {
              j ^= bit;
              bit >>= 1;
          }
          j ^= bit;
          if i < j {
              values.swap(i, j);
          }
      }
  }
  ```
- **Cache twiddle factorów** — zamiast generować `exp(-2πi k / n)` w każdej iteracji, przygotuj je raz i używaj wielokrotnie. Wektor `twiddles[k] = cos(2πk/n) - i sin(2πk/n)` umożliwia pobieranie wartości dla kolejnych etapów poprzez kroki `step = n / len`:
  ```rust
  pub fn prepare_twiddles(n: usize) -> Vec<Complex64> {
      let mut table = Vec::with_capacity(n);
      for k in 0..n {
          let angle = -2.0 * PI * (k as f64) / (n as f64);
          table.push(Complex64::from_polar(1.0, angle));
      }
      table
  }
  ```
  Podczas `fft_in_place` dla każdego bloku o długości `len` wystarczy inkrementować indeks w tablicy twiddle o `step`.
- **Odwrócona FFT i zaokrąglanie** — dla transformaty odwrotnej użyj sprzężonych twiddle factorów albo negatywnego znaku kąta, a następnie podziel wszystkie elementy przez `n`. Przy przekształceniu z powrotem na liczby całkowite zastosuj `f64::round` i konwersję do `i64`, co ograniczy błędy numeryczne.
- **Buforowane I/O oraz unikanie alokacji** — skonstruuj własny `FastScanner` pracujący na `BufRead` i przygotuj funkcję `solve_from_reader`, która alokuje bufor na dane zespolone tylko raz (przez wyzerowanie do najbliższej potęgi dwójki). To pozwoli bezpiecznie wykonywać testy graniczne.

## Ekstra podpowiedzi
- Wybierz rozmiar bufora FFT jako najmniejszą potęgę dwójki nie mniejszą niż `n + m - 1`. Ten rozmiar jest też długością tablicy twiddle factorów.
- Jeżeli podczas mnożenia wykorzystywane są zarówno wartości dodatnie, jak i ujemne, rozważ rzutowanie `i64` na `f64` w jednym miejscu, a przy powrocie pilnuj zakresu (`clamp` lub asercje).
- Sprawdź, czy wykonanie `fft_in_place` i `ifft_in_place` na kopii pustych danych zwraca identyczny wynik — to dobry test sanity check.
- Suma kontrolna: `convolution_i64(&[1], &[1])` powinna zwracać `[1]`; użyj takich mini-testów do lokalnego debugowania.
- Twój `write_answers` może korzystać z `Vec<u8>` jako bufora i na końcu wypisywać całość, co ograniczy liczbę syscalli.
