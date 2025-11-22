use b_ex_3::{ReleasePlan, ReleaseStage, StepSpec};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Szczegóły implementacji znajdziesz w README.md oraz w `lib.rs`.
    let plan = ReleasePlan::builder("Launch 1.4")
        .owner("Alice")
        .window("2024-05-10", "2024-05-12")
        .add_step(StepSpec::new(ReleaseStage::Plan, "Dry-run w stagingu"))
        .add_step(StepSpec::new(ReleaseStage::Deploy, "Włączenie flagi"))
        .build()?;

    Ok(())

}
