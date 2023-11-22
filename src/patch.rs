use crate::config;
use crate::git_utils::Git;
use crate::patch_utils;
use crate::version_utils;

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

impl Patch {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let files = config::lookable_files(&self.path);

        if files.is_empty() {
            println!("No files found");

            return Ok(());
        }

        let first_file = files.first().unwrap();
        let release = self.get_patch_version(first_file)?;

        for file in files {
            println!("Updating version {} in {:?}", &release, &file);

            if !self.dry_run {
                version_utils::change_version(&file, &release)?;
            }
        }

        let git = Git::new(&self.path, self.dry_run, self.no_verify);
        git.run(&release)?;

        Ok(())
    }

    fn get_patch_version(&self, file_path: &PathBuf) -> Result<String, Box<dyn Error>> {
        let current_version = version_utils::get_version_from_file(file_path)?;
        let patch_version = patch_utils::compute_patch_letter(&current_version);

        Ok(patch_version)
    }
}
