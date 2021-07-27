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
        short = "p",
        long = "path",
        help = "Provide the path for the node project",
        default_value = "."
    )]
    path: PathBuf,
}

impl Version {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let package_path = PathBuf::from(&self.path).join("package.json");
        let package_lock_path = PathBuf::from(&self.path).join("package-lock.json");
        let version_path = PathBuf::from(&self.path).join(PathBuf::from("./src/version.json"));

        println!("Updating version '{}' in package.json", &self.release);
        println!(
            "Updating version '{}' in package-lock.json if exists",
            &self.release
        );
        println!(
            "Updating version '{}' in ./src/version.json if exists",
            &self.release
        );
        if !self.dry_run {
            if package_path.exists() {
                self.replace(&package_path)?;
            }

            if package_lock_path.exists() {
                self.replace(&package_lock_path)?;
            }

            if version_path.exists() {
                self.replace(&version_path)?;
            }
        }
        let git = Git::new(&self.path, self.dry_run);
        git.run(&self.release)?;

        Ok(())
    }

    fn replace(&self, file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        version_utils::change_version(file_path, &self.release)?;

        Ok(())
    }
}
