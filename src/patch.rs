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
        short = "p",
        long = "path",
        help = "Provide the path for the node project",
        default_value = "."
    )]
    path: PathBuf,
}

impl Patch {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let package_path = PathBuf::from(&self.path).join("package.json");
        let package_lock_path = PathBuf::from(&self.path).join("package-lock.json");
        let version_path = PathBuf::from(&self.path).join(PathBuf::from("./src/version.json"));

        let release = self.get_patch_version(&package_path)?;

        println!("Updating version '{}' in package.json", &release);
        println!(
            "Updating version '{}' in package-lock.json if exists",
            &release
        );
        println!(
            "Updating version '{}' in ./src/version.json if exists",
            &release
        );

        if !self.dry_run {
            if package_path.exists() {
                version_utils::change_version(&package_path, &release)?;
            }

            if package_lock_path.exists() {
                version_utils::change_version(&package_lock_path, &release)?;
            }

            if version_path.exists() {
                version_utils::change_version(&version_path, &release)?;
            }
        }

        let git = Git::new(&self.path, self.dry_run);
        git.run(&release)?;

        Ok(())
    }

    fn get_patch_version(&self, file_path: &PathBuf) -> Result<String, Box<dyn Error>> {
        let current_version = version_utils::get_version_from_file(file_path)?;
        let patch_version = patch_utils::compute_patch_letter(&current_version);

        Ok(patch_version)
    }
}
