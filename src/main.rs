use std:: process;

use crate::graph::cli;

mod graph;
fn main() {
	if let Err(err) = cli::cli_interface() {
		eprintln!("Error: {err}");
		process::exit(1);
	}
}
