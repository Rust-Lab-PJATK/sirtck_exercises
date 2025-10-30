# Logarytmiczne zapytania na drzewie (`X_EX_3`)

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `X_EX_3`.
2. Na swoim branchu zaimplementuj wymagane struktury i funkcje w pliku `src/main.rs`, zastępując miejsca oznaczone `todo!()`. Nie dodawaj własnych plików.
3. Stwórz Pull Request do brancha głównego z prefiksem `X_EX_3`.
4. Po akceptacji PR lub uruchomieniu automatu otrzymasz informację zwrotną.

## Instrukcja zadania
Masz ważone drzewo o `n` wierzchołkach (`1 ≤ n ≤ 2 * 10^5`) i `q` zapytań (`1 ≤ q ≤ 2 * 10^5`). Wierzchołki są numerowane od `1` do `n`. Każda krawędź ma dodatnią wagę `w` mieszczącą się w `i64`. Dla każdego zapytania musisz wypisać wynik w osobnej linii.

- `DIST u v` — wypisz sumę wag krawędzi na prostej ścieżce pomiędzy `u` i `v`.
- `MIN u v` — wypisz najmniejszą wagę krawędzi na tej ścieżce.
- `KTH u v k` — znajdź `k`-ty wierzchołek na ścieżce od `u` do `v` licząc od `1` (czyli `k = 1` oznacza `u`, a `k = dł_ścieżki` oznacza `v`). Gwarantujemy, że `1 ≤ k ≤ liczba_wierzchołków_na_ścieżce`.

### Format wejścia
```
n q
u1 v1 w1
...
u{n-1} v{n-1} w{n-1}
zapytanie_1
...
zapytanie_q
```

### Format wyjścia
`q` linii z wynikami zapytań w kolejności ich występowania.

### Przykład
Wejście:
```
6 5
1 2 4
2 3 2
2 4 6
1 5 3
5 6 5
DIST 3 6
MIN 3 6
KTH 3 6 4
DIST 4 3
KTH 5 4 2
```
Wyjście:
```
14
2
5
8
1
```

## Wprowadzenie do nowych pojęć
- **Najniższy wspólny przodek (LCA)** — dla dowolnych `u` i `v` chcemy znaleźć wierzchołek najbliżej nich (w kierunku korzenia), który leży na obu ścieżkach do korzenia. Typowa implementacja wyznacza głębokości oraz tablicę `up[v][k]` oznaczającą przodka `v` odległego o `2^k` krawędzi:
  ```rust
  const LOG: usize = 19; // 2^19 > 2 * 10^5

  fn dfs(
      v: usize,
      parent: usize,
      adj: &[Vec<(usize, i64)>],
      depth: &mut [usize],
      up: &mut [[usize; LOG]],
      min_edge: &mut [[i64; LOG]],
      dist: &mut [i64],
  ) {
      for &(next, weight) in &adj[v] {
          if next == parent {
              continue;
          }
          depth[next] = depth[v] + 1;
          dist[next] = dist[v] + weight;
          up[next][0] = v;
          min_edge[next][0] = weight;
          for k in 1..LOG {
              let mid = up[next][k - 1];
              up[next][k] = up[mid][k - 1];
              min_edge[next][k] = min_edge[next][k - 1].min(min_edge[mid][k - 1]);
          }
          dfs(next, v, adj, depth, up, min_edge, dist);
      }
  }
  ```
- **Binary lifting z informacją o minimum** — podczas skakania o `2^k` krawędzi oprócz przodka przechowuj też minimalną wagę z tego segmentu. Funkcja podnosząca wierzchołek w górę może zwracać zarówno nowy wierzchołek, jak i minimalną wagę napotkaną po drodze:
  ```rust
  fn lift_with_min(
      mut v: usize,
      mut steps: usize,
      up: &[[usize; LOG]],
      min_edge: &[[i64; LOG]],
  ) -> (usize, i64) {
      let mut best = i64::MAX;
      for k in 0..LOG {
          if (steps & (1 << k)) != 0 {
              best = best.min(min_edge[v][k]);
              v = up[v][k];
          }
      }
      (v, best)
  }
  ```
- **K-th wierzchołek na ścieżce** — aby znaleźć `k`-ty węzeł na ścieżce `u → v`, policz długość części `u → lca` i zdecyduj, czy `k` mieści się przed `lca`, czy trzeba schodzić z `lca` w stronę `v`. Druga część wymaga skorzystania z wcześniejszych przodków `v`, ale możesz na nią spojrzeć „od końca” korzystając z głębokości i binary lifting.

## Ekstra podpowiedzi
- Zbuduj listy sąsiedztwa (`Vec<Vec<(usize, i64)>>`) i natychmiast przekształć indeksy wejściowe na 0-indeksowane — uprości to tablice i maskowanie bitowe.
- Po DFS-ie trzymaj w tablicy `dist[v]` sumę wag od korzenia; do policzenia `DIST u v` wystarczy `dist[u] + dist[v] - 2 * dist[lca]`.
- Minimalną wagę na ścieżce `u → v` wyznacz jako `min(min_do_lca(u), min_do_lca(v))`, gdzie obie wartości pochodzą z funkcji typu `lift_with_min`.
- Funkcję `kth_on_path` zaimplementuj analogicznie: najpierw zdecyduj, czy `k` leży w części rosnącej (od `u` do `lca`), czy w malejącej (od `lca` do `v`), a następnie wykonaj stosowne skoki.
- Użyj buforowanego wejścia (`BufRead`) i wypisywania (`Write`) — przy `2 * 10^5` operacji standardowe `read_line` i `println!` są za wolne.
