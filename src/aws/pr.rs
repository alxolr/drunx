
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::result::Result;
use std::{
    error::Error,
    process::{Command, Stdio},
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Create Aws Pr from command line")]
pub struct Pr {
    #[structopt(help = "Source branch name")]
    source: String,
    #[structopt(help = "Target branch name")]
    target: String,
    #[structopt(help = "Pull-request title")]
    title: String,
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Commit {
    pull_request: PullRequest,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PullRequest {
    pull_request_id: String,
}

impl Pr {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let pwd = String::from_utf8(
            Command::new("pwd")
                .current_dir(self.path.clone())
                .output()
                .expect("failed to execute process")
                .stdout,
        );

        let unwrapped = pwd?;
        let repo_name_vec = unwrapped
            .split("/")
            .last()
            .unwrap()
            .split("\n")
            .collect::<Vec<_>>();
        let repo_name = repo_name_vec.first().unwrap();

        let output = Command::new("aws")
            .args(&[
                "codecommit",
                "create-pull-request",
                "--title",
                &self.title,
                "--targets",
                &format!(
                    "repositoryName={},sourceReference={},destinationReference={}",
                    repo_name, self.source, self.target
                ),
            ])
            .stdout(Stdio::piped())
            .current_dir(&self.path.as_path())
            .output()?;

        let str_json = String::from_utf8(output.stdout)?;
        let commit: Commit = serde_json::from_str(&str_json)?;
        println!("https://console.aws.amazon.com/codesuite/codecommit/repositories/{}/pull-requests/{}/details?region=us-east-1", repo_name, commit.pull_request.pull_request_id);

        Ok(())
    }
}
