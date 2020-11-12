use std::error::Error;
use std::path::PathBuf;
use std::result::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Remove all branches merged with the local base branch")]
pub struct GitClean {
    #[structopt(help = "Branch name")]
    branch: String,
    #[structopt(
        short = "d",
        long = "dry_run",
        help = "Run the steps without executing"
    )]
    dry_run: bool,
    #[structopt(
        short = "p",
        long = "path",
        help = "Provide the path for the git project",
        default_value = "."
    )]
    path: PathBuf,
}

impl GitClean {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("working like charm");
        Ok(())
    }
}
