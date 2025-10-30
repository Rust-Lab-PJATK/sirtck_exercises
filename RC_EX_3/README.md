# Strukturalny DSL zdarzeń telemetryjnych (`RC_EX_3`)

## Jak oddać rozwiązanie
1. Stwórz branch z prefixem `RC_EX_3`.
2. Na swoim branchu zaimplementuj wymagane elementy, zastępując miejsca oznaczone `todo!()` / `unimplemented!()` oraz wpisując ciała makr w plikach `src/lib.rs` i `rc_ex_3_macros/src/lib.rs`.
3. Stwórz Pull Request do brancha głównego z prefiksem `RC_EX_3`.
4. Po akceptacji PR lub ponownym uruchomieniu automatu otrzymasz informację zwrotną.

## Instrukcja zadania
- Udostępnij typy `FieldKind`, `FieldSpec`, `EventSpec` i trait `EventBlueprint`, które opisują schemat telemetrycznego zdarzenia.
- Zaimplementuj makro `event_fields!`, które przyjmuje zapis `pole: Typ => required|optional` i zwraca referencję do statycznej tablicy `FieldSpec`.
- Zaimplementuj makro `event_spec!`, które łączy nazwę zdarzenia z makrem `event_fields!`, umożliwiając zdefiniowanie całego `EventSpec` w jednej deklaracji.
- W derive-makro `EventBlueprint` parsuj atrybuty `#[event(...)]`:
  - `#[event(name = "auth.logged_in")]` na strukturze jest wymagane.
  - `#[event(optional)]` / `#[event(required)]` na polach decydują o `FieldKind`; brak wpisu oznacza `Required`.
  - `#[event(rename = "payload.user_id")]` pozwala zmienić nazwę pola. Można łączyć z `optional/required`.
  - Nieobsługiwane parametry atrybutu i powtórzone nazwy pól powinny zgłaszać czytelny błąd kompilacji.
- Derive musi generować implementację `EventBlueprint` tworząc statyczny opis zdarzenia i korzystając z makr pomocniczych. Struktury krotek i enuma mają kończyć się błędem.
- Upewnij się, że crate `rc_ex_3` re-eksportuje derive-makro oraz makra `macro_rules!`, tak by użytkownik mógł napisać `use rc_ex_3::EventBlueprint;` i `event_spec!`.

## Wprowadzenie do nowych pojęć
- `macro_rules!` z parami nazwanych argumentów: przekazywanie zarówno identyfikatorów jak i typów.

  ```rust
  #[macro_export]
  macro_rules! tagged_values {
      ($($name:ident : $ty:ty => $value:expr),+ $(,)?) => {
          &[
              $(($crate::Tag { name: stringify!($name), ty: stringify!($ty), value: $value })),+
          ]
      };
  }
  ```

- `proc_macro::TokenStream` oraz `syn`: derive analizuje strukturę danych i atrybuty.

  ```rust
  use proc_macro::TokenStream;
  use quote::quote;
  use syn::{parse_macro_input, DeriveInput};

  #[proc_macro_derive(Demo, attributes(demo))]
  pub fn derive_demo(input: TokenStream) -> TokenStream {
      let DeriveInput { ident, .. } = parse_macro_input!(input);
      quote! {
          impl Demo for #ident {}
      }
      .into()
  }
  ```

- `quote!` i wstrzykiwanie tokenów: generowanie kodu korzystającego z makra pomocniczego.

  ```rust
  let name = syn::LitStr::new("auth.logged_in", ident.span());
  let expanded = quote! {
      const SPEC: $crate::EventSpec = $crate::event_spec!(#name, {
          user_id: u64 => required,
          session_id: Option<String> => optional,
      });
      &SPEC
  };
  ```

- Re-eksport derive-makra i funkcji pomocniczych w głównej bibliotece, aby konsumenci modułu nie musieli wiedzieć o wewnętrznej strukturze projektowej.

  ```rust
  pub use rc_ex_3_macros::EventBlueprint;
  pub use crate::macros::{event_fields, event_spec};
  ```

## Ekstra podpowiedzi
- Sprawdź duplikaty nazw pól, budując `HashSet` w derive zanim wygenerujesz kod — możesz użyć `syn::spanned::Spanned` by wskazać dokładne miejsce błędu.
- Wygodnie jest wydzielić funkcję, która mapuje `syn::Attribute` na strukturę konfiguracyjną (rename/kind), a następnie zwraca `Result`. Pozwoli to zwracać `Span::call_site()` przy złożonych błędach.
- Do generowania literalnych stringów użyj `LitStr::new`, a nazwy pól możesz pobrać `ident.to_string()`. Przy makrze `event_fields!` pamiętaj o `$(,)?` żeby dozwolić trailing comma.
- W `event_spec!` zwróć instancję `EventSpec` wprost (nie referencję) – dzięki temu derive może zainicjalizować statyczną wartość i przechowywać referencję.

## Uwaga
Podczas rozwiązywania zadania modyfikuj tylko wskazane miejsca. Zmiany w pozostałych częściach projektu mogą spowodować odrzucenie rozwiązania.
