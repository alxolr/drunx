#[macro_use]
extern crate clap;

use clap::App;
use std::error::Error;
use std::process;

mod patch;

fn run() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Drunx")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(patch::new()?)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("patch") {
        patch::run(matches)?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Drunx Cmd Errors: {}", e);
        process::exit(1);
    }
}
