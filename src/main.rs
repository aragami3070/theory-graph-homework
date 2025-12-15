mod graph;
mod tasks;

use std::process;

use crate::graph::{cli, gui::gui_interface};

fn main() -> Result<(), eframe::Error> {
    println!("Выберете вариант:");
    println!("1. 1-11 задания в cli формате");
    println!("2. Визуализация нахождения максимального потока в gui формате");
    let mut input = String::new();
    if let Err(err) = std::io::stdin().read_line(&mut input) {
        eprintln!("Error: {err}");
        process::exit(1);
    };
    let choice: u8 = input.trim().parse().unwrap();
    match choice {
        1 => {
            if let Err(err) = cli::cli_interface() {
                eprintln!("Error: {err}");
                process::exit(1);
            }
        }
        2 => gui_interface()?,
        _ => {
            eprintln!("Error: ");
            process::exit(1);
        }
    }
    Ok(())
}
