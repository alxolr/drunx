use crate::version_utils;

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::result::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Release the specific version in package.json, package-lock.json")]
pub struct Version {
    #[structopt(help = "Version tag")]
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

        println!("Adding version in package.json");
        self.replace(&package_path)?;

        println!("Changing version in package-lock.json");
        self.replace(&package_lock_path)?;

        Ok(())
    }

    fn replace(&self, file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new().read(true).open(file_path.as_path())?;

        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let version = version_utils::find(&contents);
        if version.is_some() {
            contents = contents.replace(&version.unwrap(), &self.release);
        }

        let mut file = OpenOptions::new().write(true).open(file_path.as_path())?;

        file.write_all(contents.as_bytes())?;
        file.sync_all()?;

        Ok(())
    }
}
