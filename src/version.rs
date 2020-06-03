use crate::version_utils;

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
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

        println!("Updating version '{}' in package.json", &self.release);
        println!("Updating version '{}' in package-lock.json", &self.release);
        if !self.dry_run {
            self.replace(&package_path)?;
            self.replace(&package_lock_path)?;
        }

        println!("git add . ");
        if !self.dry_run {
            self.run_git_add();
        }

        println!("git commit -a -m \"{}\"", &self.release);
        if !self.dry_run {
            self.run_git_commit();
        }

        println!(
            "git tag -a \"{}\" -m \"{}\"",
            &self.release,
            format!("Released version 'v{}'", &self.release)
        );

        if !self.dry_run {
            self.run_git_tag();
        }

        println!("git push");
        if !self.dry_run {
            self.run_git_push(false);
        }

        println!("git push --tags");
        if !self.dry_run {
            self.run_git_push(true);
        }

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

    fn run_git_add(&self) {
        Command::new("git")
            .args(&["add", "."])
            .stdout(Stdio::null())
            .current_dir(&self.path.as_path())
            .spawn()
            .expect("git add failed");
    }

    fn run_git_commit(&self) {
        Command::new("git")
            .args(&["commit", "-a", "-m", &format!("v{}", &self.release)])
            .stdout(Stdio::null())
            .current_dir(&self.path.as_path())
            .spawn()
            .expect("git commit failed");
    }

    fn run_git_tag(&self) {
        Command::new("git")
            .args(&[
                "tag",
                "-a",
                &format!("v{}", &self.release),
                "-m",
                &format!("Released version 'v{}'", &self.release),
            ])
            .stdout(Stdio::null())
            .current_dir(&self.path.as_path())
            .spawn()
            .expect("git tag failed");
    }

    fn run_git_push(&self, only_tags: bool) {
        let mut cmd = Command::new("git");

        cmd.arg("push")
            .stdout(Stdio::null())
            .current_dir(&self.path.as_path());

        if only_tags {
            cmd.arg("--tags");
        }

        cmd.spawn().expect("git push failed");
    }
}
