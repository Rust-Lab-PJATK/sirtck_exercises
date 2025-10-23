# Segmentowe minimum z leniwym dodawaniem (`X_EX_2`)

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `X_EX_2`.
2. Na swoim branchu zaimplementuj wymagane struktury i funkcje w pliku `src/main.rs`, zastępując miejsca oznaczone `todo!()`. Nie dodawaj własnych plików. 
3. Stwórz Pull Request do brancha głównego z prefiksem `X_EX_2`.
4. Po akceptacji PR lub uruchomieniu automatu otrzymasz informację zwrotną.

## Instrukcja zadania
Masz tablicę `a` długości `n` (do `2 * 10^5`) oraz `q` operacji. Wszystkie indeksy są **0-indeksowane** i pracujemy na przedziałach lewostronnie domkniętych, prawostronnie otwartych `[l, r)`. Dane są następujące typy operacji:

- `1 l r x` - dodaj wartość `x` (`|x| <= 10^9`) do każdej komórki `a[i]` dla `l <= i < r`.
- `2 l r` - wypisz minimum z przedziału `[l, r)`.

Wejście składa się z:
```
n q
a0 a1 ... a{n-1}
q wierszy z operacjami
```
Gwarantowane jest `1 <= n <= 2 * 10^5`, `1 <= q <= 2 * 10^5`, `0 <= l < r <= n`, a wszystkie wartości mieszczą się w `i64`. Wyniki zapytań typu `2` należy wypisywać po jednej wartości w linii, w kolejności występowania w wejściu.

Ponieważ łączna liczba operacji jest duża, rozwiązanie `O(n * q)` nie przejdzie testów. Wymagana jest implementacja drzewa segmentowego z leniwą propagacją (lazy propagation) oraz własnego szybkiego wczytywania danych opartego o `BufRead`.

### Przykład
Wejście:
```
5 5
3 -2 7 4 1
2 1 4
1 0 3 -5
2 0 5
1 2 5 2
2 3 5
```
Wyjście:
```
-2
-2
3
```

## Wprowadzenie do nowych pojęć
- **Szybki skaner wejścia (`FastScanner`)** - zamiast `read_line` użyj buforowanego czytnika i własnej funkcji `next`. Przykładowa struktura:
  ```rust
  use std::io::BufRead;

  pub struct FastScanner<R> {
      reader: R,
      buf: Vec<u8>,
      pos: usize,
  }

  impl<R: BufRead> FastScanner<R> {
      pub fn new(reader: R) -> Self {
          Self { reader, buf: Vec::new(), pos: 0 }
      }

      pub fn next_i64(&mut self) -> i64 {
          loop {
              if self.pos >= self.buf.len() {
                  self.buf.clear();
                  self.reader.read_until(b'\n', &mut self.buf).expect("read");
                  self.pos = 0;
              }
              while self.pos < self.buf.len() && self.buf[self.pos].is_ascii_whitespace() {
                  self.pos += 1;
              }
              if self.pos < self.buf.len() {
                  let start = self.pos;
                  while self.pos < self.buf.len() && !self.buf[self.pos].is_ascii_whitespace() {
                      self.pos += 1;
                  }
                  let token = std::str::from_utf8(&self.buf[start..self.pos]).unwrap();
                  return token.parse().unwrap();
              }
          }
      }
  }
  ```
- **Drzewo segmentowe** - przechowuj tablicę jako pełne drzewo binarne w wektorze. Każdy węzeł przechowuje minimum z reprezentowanego przedziału. Aktualizacja i zapytanie powinny działać w `O(log n)`.
  ```rust
  pub struct SegmentTree {
      size: usize,
      tree: Vec<i64>,
      lazy: Vec<i64>,
  }

  impl SegmentTree {
      pub fn from(values: &[i64]) -> Self {
          let size = values.len().next_power_of_two();
          let mut tree = vec![0; 2 * size];
          let mut lazy = vec![0; 2 * size];
          // wypełnij liście i zbuduj rodziców
          todo!("uzupełnij budowę drzewa");
      }
  }
  ```
- **Leniwa propagacja (`lazy propagation`)** - aktualizacja przedziału polega na odkładaniu informacji o dodanym przesunięciu w węzłach drzewa i propagowaniu jej dopiero, gdy jest potrzebna (np. przy zejściu w dół). Typowe helpery to `apply(node, delta)` oraz `push(node)` przenoszące zaległą aktualizację do dzieci.
  ```rust
  impl SegmentTree {
      fn apply(&mut self, node: usize, delta: i64) {
          self.tree[node] += delta;
          if node < self.size {
              self.lazy[node] += delta;
          }
      }

      fn push(&mut self, node: usize) {
          if self.lazy[node] != 0 {
              let delta = self.lazy[node];
              self.apply(node << 1, delta);
              self.apply(node << 1 | 1, delta);
              self.lazy[node] = 0;
          }
      }
  }
  ```

## Ekstra podpowiedzi
- Budowę drzewa zacznij od zaokrąglenia rozmiaru do potęgi dwójki (`next_power_of_two`), aby prościej indeksować dzieci (`node << 1` i `node << 1 | 1`).
- Przy zapytaniu o minimum pamiętaj o wcześniejszym zpropagowaniu leniwych aktualizacji na zejściu z korzenia, w przeciwnym wypadku otrzymasz nieaktualne wartości.
- Używaj typu `i64` zarówno dla wartości tablicy, jak i akumulowanych przesunięć - zakres testów wymaga obsługi wartości ujemnych oraz sumy wielu dużych aktualizacji.

## Uwaga
Podczas rozwiązywania zadania modyfikuj tylko wskazane miejsca. Zmiany w pozostałych częściach projektu mogą spowodować odrzucenie rozwiązania.
