use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    match b_ex_5::load_registry(stdin.lock()) {
        Ok(registry) => {
            #[cfg(feature = "preview")]
            {
                let preview = b_ex_5::render_preview(&registry);
                if !preview.is_empty() {
                    println!("{preview}");
                }
            }

            #[cfg(not(feature = "preview"))]
            {
                let count = registry.scopes().count();
                println!("Załadowano {count} obszarów konfiguracyjnych");
            }
        }
        Err(err) => {
            eprintln!("{err}");
        }
    }
}
