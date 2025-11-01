use b_ex_3::{ReleasePlan, ReleaseStage, StepSpec};

fn main() {
    // Szczegóły implementacji znajdziesz w README.md oraz w `lib.rs`.
    let plan = ReleasePlan::builder("Launch 1.4")
        .owner("Alice")
        .window("2024-05-10", "2024-05-12")
        .add_step(StepSpec::new(ReleaseStage::Plan, "Dry-run w stagingu"))
        .build()
        .expect("Failed to build release plan");

    plan.render_checklist().iter().for_each(|item| println!("{}", item));
}
