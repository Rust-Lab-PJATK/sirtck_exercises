# Modułowy rejestr flag (`B_EX_5`)

## Jak oddać rozwiązanie
1. Stwórz branch z prefiksem `B_EX_5`.
2. Na tym branchu uzupełnij wszystkie miejsca oznaczone `todo!()` w `src/lib.rs`, utrzymując publiczne API zgodne z poleceniem.
3. W razie potrzeby możesz modyfikować `src/main.rs`, ale nie usuwaj demonstracyjnych wywołań ani re-eksportów wymaganych w zadaniu.
4. Otwórz Pull Request do gałęzi głównej z tym samym prefiksem `B_EX_5` i poczekaj na wynik automatycznej oceny.

## Opis zadania
- Każda niepusta linia wejścia ma postać `obszar::flaga = stan`. Przed parsowaniem usuń nadmiarowe spacje wokół obu separatorów. Linie zaczynające się od `#` oraz puste linie ignoruj.
- Wspierane stany to `enabled`, `disabled` (bez względu na wielkość liter) oraz `percentage:<0-100>`. Do parsowania użyj `FeatureState::from_str`; wartości spoza zakresu powinny kończyć się `ConfigError::InvalidPercentage { raw }`, a nieznane literały `ConfigError::InvalidState { raw }`.
- `FeatureFlag::from_line` rozdziela `obszar` i `flaga` za pomocą `::`, tworzy strukturę z poprawnie sparsowanym stanem i zgłasza `ConfigError::InvalidFormat { line }`, gdy brakuje któregoś z segmentów.
- `ConfigError` musi udostępniać następujące komunikaty (`fmt::Display`): `Niepoprawny format linii: <linia>`, `Nieznany stan flagi: <wartość>`, `Niepoprawny procent: <wartość>`, `Zduplikowana flaga: <obszar>::<flaga>` oraz `Brak flag konfiguracyjnych`.
- `config::parser::parse_flags` czyta dane z `BufRead`, wykorzystuje `FeatureFlag::from_line`, pomija komentarze i puste linie oraz zwraca `ConfigError::Empty`, jeśli nie znaleziono żadnej poprawnej definicji.
- `FeatureRegistry::insert` przechowuje flagi w `BTreeMap<String, Vec<FeatureFlag>>`, dzięki czemu obszary są zawsze posortowane. W każdym obszarze przechowuj flagi w kolejności alfabetycznej po nazwie; duplikaty (`obszar` + `flaga`) muszą kończyć się `ConfigError::DuplicateFlag`.
- Udostępnij `FeatureRegistry::from_flags`, `FeatureRegistry::flags_for` (zwraca `Option<&[FeatureFlag]>`) oraz `FeatureRegistry::scopes`, które zwraca iterator `impl Iterator<Item = (&str, &[FeatureFlag])>`.
- `config::parser::load_registry` scala parsowanie z budową rejestru (`parse_flags` + `FeatureRegistry::from_flags`), propagując błędy operatorem `?`.
- `FeatureState::label` powinna zwrócić czytelną etykietę (`String`) wykorzystywaną w podglądzie; dla wartości procentowych zwróć np. `percentage:25`.
- Udostępnij moduł `prelude`, w którym zre-eksportujesz `FeatureFlag`, `FeatureState`, `FeatureRegistry`, `parse_flags` oraz `load_registry`, oraz utrzymaj analogiczne `pub use` w korzeniu crate’a.
- Funkcja `render_preview` znajduje się w `config::registry` i jest dostępna tylko, gdy włączona jest cecha kompilacyjna `preview`. Powinna zwracać zwięzły raport, np.:
  ```
  checkout::fraud -> enabled
  checkout::slow-path -> disabled
  search::beta -> percentage:25
  ```

## Nowe pojęcia
- **Zagnieżdżone moduły i porządkowanie ścieżek**  
  Duże zadania warto dzielić na mniejsze moduły. Dzięki `pub mod` możesz utrzymywać logiczną strukturę projektu.

  ```rust
  pub mod config {
      pub mod model;
      pub mod parser;
      pub mod registry;
  }
  // Użycie: crate::config::parser::parse_flags(...)
  ```

- **Re-eksport (`pub use`) i moduł `prelude`**  
  Zamiast wymagać od użytkownika znajomości wszystkich ścieżek, re-eksportuj najważniejsze typy.

  ```rust
  pub use config::model::{FeatureFlag, FeatureState, ConfigError};

  pub mod prelude {
      pub use super::{FeatureFlag, FeatureRegistry, FeatureState, load_registry, parse_flags};
  }
  ```

- **Cechy kompilacyjne i `#[cfg(feature = "...")]`**  
  Część API może być kompilowana warunkowo. W `Cargo.toml` deklarujesz cechę, a w kodzie włączasz lub wyłączasz bloki.

  ```rust
  // Cargo.toml
  [features]
  preview = []

  // lib.rs
  #[cfg(feature = "preview")]
  pub use config::registry::render_preview;
  ```

- **`impl Trait` w zwracanych iteratorach**  
  Metoda może zwrócić iterator bez ujawniania konkretnego typu, zachowując czytelny interfejs.

  ```rust
  pub fn scopes(&self) -> impl Iterator<Item = (&str, &[FeatureFlag])> {
      self.entries
          .iter()
          .map(|(scope, flags)| (scope.as_str(), flags.as_slice()))
  }
  ```

## Ekstra podpowiedzi
- Skorzystaj z `BTreeMap::entry` przy dodawaniu flag — ułatwi to wykrywanie duplikatów i utrzymanie porządku.
- Przy przetwarzaniu stanu warto najpierw sprowadzić wejście do małych liter, dzięki czemu poradzisz sobie z różnymi wariantami zapisu.
- Po dodaniu nowej flagi wywołaj `sort_by` na wewnętrznym `Vec`, aby zachować alfabet wewnątrz obszaru.
- Funkcję `render_preview` możesz zaimplementować z użyciem iteratorów i `format!`, dbając o końcowy znak nowej linii tylko wtedy, gdy rejestr nie jest pusty.

## Uwaga
Nie modyfikuj plików ani interfejsów spoza miejsc oznaczonych `todo!()`. Zmiany wykraczające poza opis zadania mogą skutkować obniżeniem oceny. Jeśli chcesz ponownie uruchomić automat oceniający, poproś administratora.
