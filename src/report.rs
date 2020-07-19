use std::error::Error;
use std::path::PathBuf;
use std::process::Command;
use std::result::Result;
use std::str;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Displays the git logs between last 2 tags")]
pub struct Report {
    #[structopt(
        short = "p",
        long = "path",
        help = "Provide the path for the git project",
        default_value = "."
    )]
    path: PathBuf,
}

impl Report {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let output = Command::new("git")
            .args(&["tag", "--sort=-creatordate"])
            .current_dir(&self.path.as_path())
            .output()
            .expect("git tag --sort=-creatordate failed");

        let revisions = str::from_utf8(output.stdout.as_slice())?
            .split("\n")
            .take(2)
            .collect::<Vec<_>>();

        println!(
            "git log --pretty=format:%s {}..{}",
            revisions[1], revisions[0]
        );

        let reports_bin = Command::new("git")
            .args(&[
                "log",
                "--pretty=format:%s",
                &format!("{}..{}", revisions[1], revisions[0]),
            ])
            .current_dir(&self.path.as_path())
            .output()
            .expect("git log failed");

        let reports_str = str::from_utf8(reports_bin.stdout.as_slice())?
            .split("\n")
            .filter(|c| c != &revisions[0])
            .map(|c| format!(" - {}", c))
            .collect::<Vec<_>>()
            .join("\n");

        println!("{}", reports_str);

        Ok(())
    }
}
