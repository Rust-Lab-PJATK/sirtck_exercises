# Plan wydania (`B_EX_3`)

## Jak oddać rozwiązanie
1. Stwórz branch z prefiksem `B_EX_3`.
2. Na tym branchu uzupełnij wszystkie miejsca oznaczone `todo!()` w `src/lib.rs`, nie zmieniaj interfejsów publicznych.
3. Jeśli potrzebujesz modyfikować `src/main.rs`, rób to z zachowaniem interfejsów biblioteki (plik jest tylko demonstracją i nie będzie oceniany).
4. Otwórz Pull Request do gałęzi głównej z tym samym prefiksem `B_EX_3`.
5. Zaczekaj na ocenę automatu lub recenzenta.

## Opis zadania
- Zaimplementuj domenę planu wydania produktu. Trzy etapy (`PLAN`, `DEPLOY`, `VERIFY`) muszą być reprezentowane w `enum` i wykorzystywane w raportach.
- Udostępnij wzorzec builder w postaci `ReleasePlan::builder`, który pozwoli ustawić nazwę, właściciela (`owner`), okno czasowe (`DateWindow::new`) oraz dodać kroki (`StepSpec`).
- Metody buildera powinny zwracać `Self`, aby można było je łańcuchować (`builder.owner(...).window(...).add_step(...)`).
- Waliduj dane przy `build()`: brak właściciela, brak okna czasowego lub brak kroków powinny kończyć się wariantem `BuildError`. Przy krokach bez właściciela użyj właściciela planu, a jeśli go nie ma – zgłoś błąd.
- Udostępnij `ReleasePlan::render_checklist`, która zwróci linie w kolejności etapów: najpierw nagłówek z nazwą i oknem czasowym, następnie wszystkie kroki `PLAN`, potem `DEPLOY`, na końcu `VERIFY` (z opisem i właścicielem).

## Nowe pojęcia
- **Builder w praktyce**  
  Zamiast wielkiego konstruktora z dziesiątką argumentów, budujesz obiekt krocząc po danych. Każda metoda zmienia stan buildera i zwraca `Self`, dzięki czemu można łańcuchować wywołania:

  ```rust
  let plan = ReleasePlan::builder("Launch 1.4")
      .owner("Alice")
      .window("2024-05-10", "2024-05-12")
      .add_step(StepSpec::new(ReleaseStage::Plan, "Dry-run w stagingu"))
      .add_step(StepSpec::new(ReleaseStage::Deploy, "Włączenie flagi"))
      .build()?;
  ```

- **Metody pomocnicze w blokach `impl`**  
  Własne funkcje (`label`, `steps_for`, `render_checklist`) umieszczone przy typach utrzymują logikę blisko danych. Dzięki temu testy mogą korzystać z czytelnych API, a kod pozostaje zwarty.

- **Walidacja podczas `build()`**  
  Builder jest dobrym miejscem na weryfikację wymaganych pól. Gdy czegoś brakuje, zwracasz `Result::Err(BuildError::MissingOwner)` zamiast tworzyć niekompletny obiekt.

## Ekstra podpowiedzi
- Zadbaj o kolejność etapów, korzystając z `ReleaseStage::all()` i filtrowania kroków dla każdego wariantu.
- Przy łączeniu tesktów pomocny będzie `format!`, natomiast do kopiowania właściciela użyj `clone` tylko tam, gdzie to konieczne.
- Etapy bez kroków pomiń w raporcie – wyświetlaj tylko to, co faktycznie ma się wydarzyć.
- W `build()` możesz zmapować `StepSpec` na `ReleaseStep` używając iteratorów (`into_iter().map(...)`), co uprości kod.

## Uwaga
Nie modyfikuj plików ani interfejsów spoza miejsc oznaczonych `todo!()`. Nadpisywanie istniejącej struktury lub zmiana funkcji publicznych może skutkować obniżeniem oceny. Jeśli potrzebujesz ponownie uruchomić automat, poproś administratora.
