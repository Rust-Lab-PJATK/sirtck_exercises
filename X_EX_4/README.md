# Maksymalny przepływ i minimalny przekrój (`X_EX_4`)

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `X_EX_4`.
2. Na swoim branchu zaimplementuj wymagane struktury i funkcje w pliku `src/main.rs`, zastępując miejsca oznaczone `todo!()`. Nie dodawaj własnych plików.
3. Stwórz Pull Request do brancha głównego z prefiksem `X_EX_4`.
4. Po akceptacji PR lub uruchomieniu automatu otrzymasz informację zwrotną.

## Instrukcja zadania
Masz skierowany graf o `n` wierzchołkach (`1 ≤ n ≤ 5 * 10^4`) i `m` krawędziach (`1 ≤ m ≤ 2 * 10^5`). Każda krawędź ma dodatnią pojemność w przedziale `1..=10^12`. Krawędzie są numerowane w kolejności wejścia od `1`. Wierzchołki są 1-indeksowane. Należy policzyć wartość maksymalnego przepływu pomiędzy wierzchołkami `s` i `t`, a następnie wskazać minimalny przekrój wyznaczony przez zbiór `S` wierzchołków osiągalnych z `s` w grafie residualnym.

Twoje wyjście powinno mieć trzy linie:
1. `max_flow` — pojedyncza liczba całkowita (`i64`), wartość maksymalnego przepływu.
2. `k v1 v2 ... vk` — liczba `k = |S|` oraz posortowana rosnąco lista wierzchołków z `S` (numeracja wejściowa, 1-indeksowana).
3. `c e1 e2 ... ec` — liczba `c` oraz posortowana rosnąco lista identyfikatorów krawędzi, które wychodzą z `S` do `V \\ S` (minimalny przekrój). Krawędź należy wypisać, jeśli w oryginalnym grafie prowadziła z wierzchołka z `S` do wierzchołka spoza `S`.

Wejście:
```
n m s t
u1 v1 c1
...
um vm cm
```

Wyjście:
```
max_flow
k v1 ... vk
c e1 ... ec
```

### Przykład
Wejście:
```
4 5 1 4
1 2 5
1 3 4
2 3 2
2 4 3
3 4 4
```

Wyjście:
```
7
2 1 2
2 2 4
```

Komentarz: Maksymalny przepływ ma wartość `7`. W grafie residualnym z `s = 1` osiągalne są wierzchołki `{1, 2}`. Krawędzie przekroju (w numeracji wejściowej) to `2` (`1 → 3`) oraz `4` (`2 → 4`).

## Wprowadzenie do nowych pojęć
- **Algorytm Dinica (level graph + blocking flow)** — budujemy poziomy graf z BFS oraz przepychamy przepływ za pomocą DFS, zatrzymując się na krawędziach bez pojemności residualnej. W praktyce warto przechowywać graf w postaci:
  ```rust
  pub struct Edge {
      pub to: usize,
      pub rev: usize, // indeks krawędzi odwrotnej w sąsiedztwie `to`
      pub capacity: i64,
  }

  pub struct Dinic {
      pub graph: Vec<Vec<Edge>>,
      pub level: Vec<i32>,
      pub iter: Vec<usize>,
  }
  ```
  Dodając krawędź `u -> v`, pamiętaj o wstawieniu również krawędzi odwrotnej o początkowej pojemności `0`.
- **Optymalizacja current-arc** — przy DFS blokującym pamiętaj o przechowywaniu wskaźnika `iter[u]`, który mówi, od której krawędzi powinniśmy kontynuować szukanie ścieżek. Dzięki temu każda krawędź jest badana co najwyżej raz na poziom.
  ```rust
  fn dfs(&mut self, v: usize, t: usize, f: i64) -> i64 {
      if v == t {
          return f;
      }
      for i in self.iter[v]..self.graph[v].len() {
          self.iter[v] = i;
          let edge = self.graph[v][i].clone();
          if edge.capacity > 0 && self.level[v] < self.level[edge.to] {
              let pushed = self.dfs(edge.to, t, f.min(edge.capacity));
              if pushed > 0 {
                  // zaktualizuj krawędź oraz rewers
                  todo!("zmodyfikuj pojemności residualne");
                  return pushed;
              }
          }
      }
      0
  }
  ```
- **Minimalny przekrój z grafu residualnego** — po policzeniu maksimum wykonaj BFS/DFS po krawędziach residualnych (`capacity > 0`) od `s`. Otrzymasz zbiór `S`. Aby zbudować listę krawędzi przekroju, przejdź po oryginalnych krawędziach i wybierz te, których początek należy do `S`, a koniec już nie.
- **Szybkie wejście/wyjście** — przy tak dużych `m` standardowe `read_line` i `println!` są niewystarczające. Zaimplementuj własny skaner opary o `BufRead` i wypisywanie z `Write`.

## Ekstra podpowiedzi
- Indeksuj wierzchołki wewnętrznie od `0`; na końcu pamiętaj o ponownym zmapowaniu do numeracji wejściowej przed wypisywaniem wyniku.
- Trzymaj identyfikatory krawędzi w osobnej tablicy równoległej do twojej listy krawędzi — ułatwi to generowanie listy przekroju.
- Jeśli przepustowości mieszczą się w `i64`, to sumaryczny przepływ także się zmieści (max `m * 10^12`), ale przy dodawaniu zwracaj uwagę na przepełnienia — używaj `i128` w akumulacji, gdy to upraszcza kod.
- Krawędzie wielokrotne są dozwolone. Nie filtruj ich — każda ma własny identyfikator i może trafić do przekroju.
- Weź pod uwagę grafy, w których `s` nie osiąga `t`. Wtedy maksymalny przepływ to `0`, a `S` może zawierać tylko `s`.
