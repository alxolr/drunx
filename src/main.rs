mod version_utils;

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Write};
use std::process;

fn run() -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open("data/package.json")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let version = version_utils::find(&contents);
    if version.is_some() {
        contents = contents.replace(&version.unwrap(), "3.0.0a");
    }

    let mut file = OpenOptions::new().write(true).open("data/package.json")?;

    file.write_all(contents.as_bytes())?;
    file.sync_all()?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Drunx Cmd Errors: {}", e);
        process::exit(1);
    }
}
