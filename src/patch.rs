use std::error::Error;
use std::path::PathBuf;
use std::result::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Change the package.json, package-lock.json with the needed patch version")]
pub struct Patch {
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
}

impl Patch {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("hello world");

        Ok(())
    }
}
