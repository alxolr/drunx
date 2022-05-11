mod aws;
mod git;
mod git_utils;
mod notify_utils;
mod patch;
mod patch_utils;
mod version;
mod version_utils;

use aws::Aws;
use git::Git;
use notify_utils::{Icon, Notification, Urgency};
use patch::Patch;
use std::error::Error;
use std::process;
use structopt::StructOpt;
use version::Version;

#[derive(StructOpt, Debug)]
#[structopt(
    version = "1.7.0",
    about = "Productivity commands",
    rename_all = "kebab-case"
)]
enum Drunx {
    Version(Version),
    Patch(Patch),
    Aws(Aws),
    Git(Git),
}

fn run() -> Result<(), Box<dyn Error>> {
    let cmd = Drunx::from_args();
    match cmd {
        Drunx::Version(version) => version.run()?,
        Drunx::Patch(patch) => patch.run()?,
        Drunx::Aws(aws) => aws.run()?,
        Drunx::Git(git) => git.run()?,
    }

  
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Drunx Cmd Errors: {}", e);
        process::exit(1);
    }
}
