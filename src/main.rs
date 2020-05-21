mod version_utils;
use structopt::StructOpt;

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::process;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Productivity scripts for deployment",
    rename_all = "kebab-case"
)]
enum Drunx {
    Patch {
        #[structopt(help = "Sprint number used in the created tag")]
        sprint: i32,
        #[structopt(
            short = "d",
            long = "dry_run",
            help = "Run the steps without executing"
        )]
        dry_run: bool,
        #[structopt(
            short = "p",
            long = "path",
            help = "Provide the path for the node project",
            default_value = "."
        )]
        path: PathBuf,
    },
}

fn run() -> Result<(), Box<dyn Error>> {
    let drunx = Drunx::from_args();
    println!("{:?}", drunx);

    // let file = OpenOptions::new().read(true).open("data/package.json")?;
    // let mut buf_reader = BufReader::new(file);
    // let mut contents = String::new();
    // buf_reader.read_to_string(&mut contents)?;

    // let version = version_utils::find(&contents);
    // if version.is_some() {
    //     contents = contents.replace(&version.unwrap(), "3.0.0a");
    // }

    // let mut file = OpenOptions::new().write(true).open("data/package.json")?;

    // file.write_all(contents.as_bytes())?;
    // file.sync_all()?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Drunx Cmd Errors: {}", e);
        process::exit(1);
    }
}
