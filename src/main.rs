mod version_finder;

use std::error::Error;
use std::process;

fn run() -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Drunx Cmd Errors: {}", e);
        process::exit(1);
    }
}
