use crate::config::lookable_files;
use crate::git_utils::Git;
use crate::version_utils;

use std::error::Error;
use std::path::PathBuf;
use std::result::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Release the specified version in package.json, package-lock.json\nadds a release commit and an anotated tag"
)]
pub struct Version {
    #[structopt(help = "Semver version")]
    release: String,
    #[structopt(
        short = "d",
        long = "dry_run",
        help = "Run the steps without executing"
    )]
    dry_run: bool,
    #[structopt(
        short = "n",
        long = "no_verify",
        help = "Run the git command without applying the git hooks"
    )]
    no_verify: bool,
    #[structopt(
        short = "p",
        long = "path",
        help = "Provide the path for the node project",
        default_value = "."
    )]
    path: PathBuf,
}

impl Version {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let files = lookable_files(&self.path);

        if files.is_empty() {
            println!("No files found");

            return Ok(());
        }

        for file in files {
            println!("Updating version {} in {:?}", &self.release, &file);

            if !self.dry_run {
                version_utils::change_version(&file, &self.release)?;
            }
        }

        let git = Git::new(&self.path, self.dry_run, self.no_verify);
        git.run(&self.release)?;

        Ok(())
    }
}
