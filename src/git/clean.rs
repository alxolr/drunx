use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::result::Result;
use std::str;
use structopt::StructOpt;

use crate::git_utils::Git;

#[derive(StructOpt, Debug)]
#[structopt(about = "Git Clean local branches")]
pub struct Clean {
    #[structopt(help = "Provide the root branch")]
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

impl Clean {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        Git::new(&self.path, self.dry_run).git_fetch(true)?;

        let remote_branches = self.get_remote_branches()?;
        let local_merged_branches = self.get_local_merged_branches()?;

        for branch in local_merged_branches {
            if !remote_branches.contains(&branch) {
                println!("git branch -D {}", branch);

                if !self.dry_run {
                    Command::new("git")
                        .args(&["branch", "-D", &branch])
                        .current_dir(&self.path.as_path())
                        .spawn()
                        .expect("git delete failed")
                        .wait()
                        .expect("remove branch did not work");
                }
            }
        }

        Ok(())
    }

    fn get_remote_branches(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let output = Command::new("git")
            .args(&["branch", "-r"])
            .stdout(Stdio::piped())
            .current_dir(&self.path.as_path())
            .output()?;

        let remote_branches = str::from_utf8(output.stdout.as_slice())?
            .split("\n")
            .map(|branch| branch.trim().replace("origin/", "").clone())
            .map(|branch| branch.replace("* ", ""))
            .filter(|branch| branch != "")
            .collect::<Vec<_>>();

        Ok(remote_branches)
    }

    // filter out all the branches already merged with next branch
    fn get_local_merged_branches(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let output = Command::new("git")
            .args(&["branch", "--merged", &format!("origin/{}", self.branch)])
            .stdout(Stdio::piped())
            .current_dir(&self.path.as_path())
            .output()?;

        let merged_branches = str::from_utf8(output.stdout.as_slice())?
            .split("\n")
            // remove the "* " which marks the current checkout branch
            .map(|line| line.trim().replace("* ", "").clone())
            // filter from the list the branch which we consider the main
            .filter(|branch| branch != "" && branch != &self.branch)
            .collect::<Vec<_>>();

        Ok(merged_branches)
    }
}
