# Szyna komend z dynamiczną rejestracją (`C_EX_3`)

## Jak oddać rozwiązanie
- Stwórz branch z prefixem `C_EX_3`.
- Na swoim branchu zaimplementuj brakujące fragmenty w pliku `src/lib.rs`, zastępując wszystkie wywołania `todo!()`.
- Stwórz Pull Request do głównego repozytorium z prefiksem `C_EX_3` w nazwie.
- Poczekaj na wynik automatycznej oceny lub review.

## Instrukcja zadania
- Zdefiniuj trait `Command` z powiązanym typem `Output`, opisującym wynik wykonania komendy. W testsach komendy będą prostymi strukturami (`struct AddUser { ... }`) i oczekuje się, że `Output` przyjmie konkretny typ (np. `Result<String, Error>`).
- Zaimplementuj trait `CommandHandler<C>` z metodą `handle(&self, &C) -> C::Output`. Domyślna implementacja dla funkcji/closure (`impl<F> CommandHandler<C> for F`) znajduje się już w szkielecie — nie usuwaj jej, możesz z niej korzystać.
- Zbuduj strukturę `CommandBus`, która przechowuje handlerów w `HashMap<TypeId, HandlerEntry>`. Rejestrowanie handlera (`register::<C, H>`) powinno działać tylko, jeśli dla danej komendy nie ma jeszcze innego handlera — w przeciwnym razie zwróć `DispatchError::HandlerAlreadyRegistered`.
- W metodzie `execute::<C>(&self, &C)` wyszukaj handler po `TypeId::of::<C>()`, uruchom go i zrzutuj wynik do `C::Output`. Jeśli handler nie istnieje, zwróć `DispatchError::HandlerNotFound`. W przypadku błędu rzutowania zwróć `DispatchError::WrongCommandType` (komenda) lub `DispatchError::IncompatibleOutput` (wynik).
- `CommandBus::contains::<C>()` ma zwracać `true`, gdy handler dla `C` jest zarejestrowany. Wykorzystaj identyfikatory typów, aby uniknąć stringowego porównywania.
- `TypedHandler<C, H>` i `Box<dyn ErasedHandler>` służą do przechowywania handlerów w postaci obiektów `dyn`. Uzupełnij implementację `ErasedHandler` tak, aby bezpiecznie rzutowała komendę i przekazywała wynik jako `Box<dyn Any>`.
- Metody `register` i `execute` powinny działać wyłącznie dla komend i wyników spełniających `'static`, ponieważ typerowanie opiera się o `Any`. Upewnij się, że odpowiednie ograniczenia znajdują się w sygnaturach metod.

## Nowe pojęcia i przypomnienia
- **Associated Types** – traity mogą definiować powiązane typy dostępne pod nazwą `TypeName`. To często stosowany sposób na opisanie wyników operacji. Przykład:

  ```rust
  trait Repository {
      type Item;

      fn find_all(&self) -> Vec<Self::Item>;
  }
  ```

  W naszym zadaniu `Command::Output` jest właśnie takim powiązanym typem.

- **Trait objects (`dyn Trait`) i wymazywanie typów** – przechowanie wartości o wielu typach w jednej kolekcji wymaga obiektów `dyn`. Wewnętrzny `HandlerEntry` trzyma `Box<dyn ErasedHandler>`, aby każda struktura implementująca odpowiedni trait mogła być wywołana w jednolity sposób:

  ```rust
  let object: Box<dyn std::fmt::Display> = Box::new("tekst");
  println!("{}", object); // działa mimo że nie znamy konkretnego typu
  ```

- **`std::any::Any` i `TypeId`** – pozwalają na sprawdzanie typu w czasie działania. Dzięki temu możemy upewnić się, że handler otrzymuje właściwą komendę:

  ```rust
  use std::any::Any;

  fn try_ref<T: 'static>(value: &dyn Any) -> Option<&T> {
      value.downcast_ref::<T>()
  }
  ```

  W `CommandBus` użyjesz `TypeId::of::<C>()` jako klucza w `HashMap`, a `Any::downcast_ref`/`Any::downcast::<T>` do rzutowania.

- **`PhantomData`** – marker typów wykorzystywany, gdy struktura logicznie zależy od typu generycznego, ale go fizycznie nie przechowuje. `TypedHandler<C, H>` korzysta z `PhantomData<C>`, aby kompilator śledził, dla jakiej komendy dany handler jest przeznaczony.

- **Ograniczenie `'static`** – jeżeli typ ma być przechowywany w `Any`, musi spełniać `'static`. To oznacza, że nie może zawierać krótkotrwałych referencji. W praktyce większość struktur z własnymi danymi (`String`, `Vec`, itp.) spełnia to wymaganie.

## Ekstra podpowiedzi
- Utrzymuj w `HandlerEntry` nazwy typów (`type_name::<C>()`) — ułatwi to tworzenie czytelnych komunikatów w `DispatchError`.
- `HashMap::insert` zwraca poprzednią wartość – wykorzystaj to, by wykryć duplikaty rejestracji i zawrócić z błędem.
- Przy rzutowaniu wyniku z `Box<dyn Any>` możesz użyć `downcast::<C::Output>()`, które zwraca `Result<Box<C::Output>, Box<dyn Any>>`. Pamiętaj o rozpakowaniu `Box` do wartości (`*boxed_output`).
- Testy oczekują, że `CommandBus::execute` nie kopiuje komendy – metoda przyjmuje ją przez referencję. Przechowuj handlerów jako `&self`/`&C`, aby zachować pożyczanie.
- Jeśli potrzebujesz więcej diagnostyki przy debugowaniu, tymczasowo loguj nazwy typów i kluczy (`dbg!(type_name::<C>())`), a przed oddaniem rozwiązania usuń dodatkowe `dbg!`.
