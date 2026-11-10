
use b_ex_3::ReleasePlan;
use b_ex_3::ReleaseStage;
use b_ex_3::StepSpec;

fn main() {
    let plan = ReleasePlan::builder("Launch 1.4")
        .owner("Alice")
        .window("2024-05-10", "2024-05-12")
        .add_step(StepSpec::new(ReleaseStage::Plan, "Dry-run w stagingu"))
        .add_step(StepSpec::new(ReleaseStage::Deploy, "Włączenie flagi"))
        .build();
    match plan {
        Ok(plan) => {
            for line in plan.render_checklist() {
                println!("{}", line);
            }
        },
        Err(err) => {
            println!("{}", err);
        }
    };

}
