use std::io::{self};

use b_ex_2::run_from_reader;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    match run_from_reader(handle) {
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
