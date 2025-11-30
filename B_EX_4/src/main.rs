use std::io::{self};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    match b_ex_4::collect_report(reader) {
        Ok(lines) => {
            for line in lines {
                println!("{line}");
            }
        }
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
