//! Biblioteka do budowania planu wydania z użyciem wzorca builder.
use std::fmt;

pub use release::{
    BuildError, DateWindow, ReleasePlan, ReleasePlanBuilder, ReleaseStage, ReleaseStep, StepSpec,
};

/// Logika domenowa planu wydania.
pub mod release {
    use super::fmt;

    /// Etapy, które muszą pojawić się w planie i określają kolejność kroków.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum ReleaseStage {
        Plan,
        Deploy,
        Verify,
    }

    impl ReleaseStage {
        /// Wszystkie etapy w kolejności wymaganej przez `render_checklist`.
        pub fn all() -> [ReleaseStage; 3] {
            todo!("zwróć tablicę z wariantami w kolejności PLAN -> DEPLOY -> VERIFY")
        }

        /// Etykieta wykorzystywana w komunikatach tekstowych.
        pub fn label(self) -> &'static str {
            todo!("zwróć krótką nazwę etapu typu PLAN / DEPLOY / VERIFY")
        }
    }

    impl fmt::Display for ReleaseStage {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            todo!("wykorzystaj ReleaseStage::label w implementacji Display")
        }
    }

    /// Okno czasowe, w którym realizujemy wydanie.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DateWindow {
        pub start: String,
        pub end: String,
    }

    impl DateWindow {
        /// Tworzy okno czasowe z datą startu oraz końca.
        pub fn new(start: impl Into<String>, end: impl Into<String>) -> Self {
            todo!("zapisz przekazane wartości w polach struktury")
        }
    }

    /// Kroki, które należy wykonać na danym etapie.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ReleaseStep {
        pub stage: ReleaseStage,
        pub description: String,
        pub owner: String,
    }

    /// Dane wejściowe pojedynczego kroku wykorzystywane przez builder.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct StepSpec {
        stage: ReleaseStage,
        description: String,
        owner: Option<String>,
    }

    impl StepSpec {
        /// Buduje specyfikację kroku bez przypisanego właściciela.
        pub fn new(stage: ReleaseStage, description: impl Into<String>) -> Self {
            todo!("zainicjuj strukturę z opisem i etapem, pozostawiając owner == None")
        }

        /// Ustawia właściciela kroku, zwracając zmodyfikowaną specyfikację.
        pub fn with_owner(mut self, owner: impl Into<String>) -> Self {
            todo!("zapisz przekazane imię/alias właściciela i zwróć Self")
        }

        /// Zwraca etap kroku (potrzebne w testach i przy budowaniu planu).
        pub fn stage(&self) -> ReleaseStage {
            todo!("zwróć etap z pola stage")
        }

        /// Opis kroku bez modyfikacji.
        pub fn description(&self) -> &str {
            todo!("daj dostęp do opisu kroku")
        }

        /// Zwraca właściciela, jeśli został ustawiony.
        pub fn owner(&self) -> Option<&str> {
            todo!("zwróć opcjonalnego ownera jako &str")
        }
    }

    /// Błąd budowania planu.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum BuildError {
        /// Nie ustawiono głównego właściciela planu.
        MissingOwner,
        /// Nie ustawiono okna czasowego.
        MissingWindow,
        /// Nie dodano żadnych kroków.
        NoSteps,
        /// Krok bez właściciela w sytuacji, gdy plan też nie ma właściciela.
        MissingStepOwner { description: String },
    }

    impl fmt::Display for BuildError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            todo!("zamień warianty na zrozumiałe komunikaty dla użytkownika")
        }
    }

    impl std::error::Error for BuildError {}

    /// Gotowy plan wydania.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ReleasePlan {
        pub name: String,
        pub owner: String,
        pub window: DateWindow,
        pub steps: Vec<ReleaseStep>,
    }

    impl ReleasePlan {
        /// Tworzy builder, który pozwoli zebrać wszystkie dane planu.
        pub fn builder(name: impl Into<String>) -> ReleasePlanBuilder {
            todo!("zainicjuj builder z nazwą planu")
        }

        /// Zwraca kroki danego etapu w kolejności dodania.
        pub fn steps_for(&self, stage: ReleaseStage) -> Vec<&ReleaseStep> {
            todo!("przefiltruj kroki po etapie i zachowaj ich oryginalną kolejność")
        }

        /// Generuje listę linii gotową do wypisania w CLI.
        ///
        /// Pierwsza linia zawiera nazwę planu i okno czasowe, kolejne linie to kroki `PLAN`, potem
        /// `DEPLOY`, a na końcu `VERIFY`. Przy każdym kroku wypisz opis oraz właściciela.
        pub fn render_checklist(&self) -> Vec<String> {
            todo!("zgrupuj kroki według etapu i zbuduj oczekiwane linie tekstu")
        }
    }

    /// Builder zbierający wszystkie informacje potrzebne do stworzenia [`ReleasePlan`].
    #[derive(Debug, Clone)]
    pub struct ReleasePlanBuilder {
        name: String,
        owner: Option<String>,
        window: Option<DateWindow>,
        steps: Vec<StepSpec>,
    }

    impl ReleasePlanBuilder {
        /// Utwórz builder z nazwą planu (wykorzystywane przez `ReleasePlan::builder`).
        pub fn new(name: impl Into<String>) -> Self {
            todo!("zapisz nazwę, a pozostałe pola ustaw na wartości domyślne")
        }

        /// Ustaw właściciela całego planu. Możesz wywołać wielokrotnie, ostatnia wartość wygrywa.
        pub fn owner(mut self, owner: impl Into<String>) -> Self {
            todo!("zapisz ownera i zwróć builder dla chainingu")
        }

        /// Określ okno czasowe wydania.
        pub fn window(mut self, start: impl Into<String>, end: impl Into<String>) -> Self {
            todo!("stwórz DateWindow i dodaj go do buildera")
        }

        /// Dodaj krok do planu (kolejność dodania jest zachowywana w obrębie etapu).
        pub fn add_step(mut self, step: StepSpec) -> Self {
            todo!("dodaj specyfikację do bufora i zwróć builder")
        }

        /// Finalizuje builder i zwraca gotowy plan lub błąd walidacji.
        pub fn build(self) -> Result<ReleasePlan, BuildError> {
            todo!(
                "sprawdź wymagane pola, uzupełnij brakujących właścicieli i zbuduj ReleasePlan"
            )
        }
    }
}
