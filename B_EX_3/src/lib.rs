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
            [
                ReleaseStage::Plan,
                ReleaseStage::Deploy,
                ReleaseStage::Verify,
            ]
        }

        /// Etykieta wykorzystywana w komunikatach tekstowych.
        pub fn label(self) -> &'static str {
            match self {
                ReleaseStage::Plan => "PLAN",
                ReleaseStage::Deploy => "DEPLOY",
                ReleaseStage::Verify => "VERIFY",
            }
        }
    }

    impl fmt::Display for ReleaseStage {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.label())
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
            Self {
                start: start.into(),
                end: end.into(),
            }
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
            Self {
                stage,
                description: description.into(),
                owner: None,
            }
        }

        /// Ustawia właściciela kroku, zwracając zmodyfikowaną specyfikację.
        pub fn with_owner(mut self, owner: impl Into<String>) -> Self {
            self.owner = Some(owner.into());
            self
        }

        /// Zwraca etap kroku (potrzebne w testach i przy budowaniu planu).
        pub fn stage(&self) -> ReleaseStage {
            self.stage
        }

        /// Opis kroku bez modyfikacji.
        pub fn description(&self) -> &str {
            &self.description
        }

        /// Zwraca właściciela, jeśli został ustawiony.
        pub fn owner(&self) -> Option<&str> {
            self.owner.as_deref()
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
            match self {
                BuildError::MissingOwner => write!(f, "Plan nie ma ustawionego właściciela"),
                BuildError::MissingWindow => write!(f, "Plan nie ma ustawionego okna czasowego"),
                BuildError::NoSteps => write!(f, "Plan nie zawiera żadnych kroków"),
                BuildError::MissingStepOwner { description } => write!(
                    f, "Krok '{}' nie ma ustawionego właściciela", description
                )
            }
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
            ReleasePlanBuilder::new(name)
        }

        /// Zwraca kroki danego etapu w kolejności dodania.
        pub fn steps_for(&self, stage: ReleaseStage) -> Vec<&ReleaseStep> {
            self.steps.iter().filter(|step| step.stage == stage).collect()
        }

        /// Generuje listę linii gotową do wypisania w CLI.
        ///
        /// Pierwsza linia zawiera nazwę planu i okno czasowe, kolejne linie to kroki `PLAN`, potem
        /// `DEPLOY`, a na końcu `VERIFY`. Przy każdym kroku wypisz opis oraz właściciela.
        pub fn render_checklist(&self) -> Vec<String> {
            let mut lines = Vec::new();
            lines.push(format!(
                "Release Plan: {} (Window: {} - {})",
                self.name, self.window.start, self.window.end
            ));

            for stage in ReleaseStage::all() {
                let stage_steps = self.steps_for(stage);
                for step in stage_steps {
                    lines.push(format!(
                        "[{}] {} (Owner: {})",
                        stage,
                        step.description,
                        step.owner
                    ))
                }
            }
            lines
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
            Self {
                name: name.into(),
                owner: None,
                window: None,
                steps: Vec::new(),
            }
        }

        /// Ustaw właściciela całego planu. Możesz wywołać wielokrotnie, ostatnia wartość wygrywa.
        pub fn owner(mut self, owner: impl Into<String>) -> Self {
            self.owner = Some(owner.into());
            self
        }

        /// Określ okno czasowe wydania.
        pub fn window(mut self, start: impl Into<String>, end: impl Into<String>) -> Self {
            self.window = Some(DateWindow::new(start, end));
            self
        }

        /// Dodaj krok do planu (kolejność dodania jest zachowywana w obrębie etapu).
        pub fn add_step(mut self, step: StepSpec) -> Self {
            self.steps.push(step);
            self
        }

        /// Finalizuje builder i zwraca gotowy plan lub błąd walidacji.
        pub fn build(self) -> Result<ReleasePlan, BuildError> {
            let window = self.window.ok_or(BuildError::MissingWindow)?;
            if self.steps.is_empty() {
                return Err(BuildError::NoSteps);
            }

            let mut release_steps = Vec::new();
            for step_spec in self.steps {
                let owner = match step_spec.owner() {
                    Some(owner) => owner.to_string(),
                    None => match &self.owner {
                        Some(plan_owner) => plan_owner.clone(),
                        None => {
                            return Err(BuildError::MissingStepOwner {
                                description: step_spec.description().to_string(),
                            })
                        }
                    }
                };

                release_steps.push(ReleaseStep {
                    stage: step_spec.stage(),
                    description: step_spec.description().to_string(),
                    owner,
                });
            }

            let plan_owner = self.owner.ok_or(BuildError::MissingOwner)?;

            Ok(ReleasePlan {
                name: self.name,
                owner: plan_owner,
                window,
                steps: release_steps,
            })
        }
    }
}
