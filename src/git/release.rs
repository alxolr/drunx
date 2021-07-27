use crate::git_utils::Git;

use std::error::Error;
use std::path::PathBuf;
use std::result::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Hard reset the code from one branch to other")]
pub struct Release {
    #[structopt(help = "From branch, will pick origin/{branch}")]
    from: String,
    #[structopt(help = "To branch, will pick origin/{branch}")]
    to: String,
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

impl Release {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let git = Git::new(&self.path, self.dry_run);
        git.run_release(&self.from, &self.to)?;

        Ok(())
    }
}
