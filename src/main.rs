mod graph;
mod tasks;

use std::process;

use crate::graph::cli;

fn main() {
    if let Err(err) = cli::cli_interface() {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
