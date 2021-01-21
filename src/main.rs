mod clean_branch;
mod git;
mod patch;
mod patch_utils;
mod release;
mod report;
mod version;
mod version_utils;

use clean_branch::CleanBranch;
use patch::Patch;
use release::Release;
use report::Report;
use std::error::Error;
use std::process;
use structopt::StructOpt;
use version::Version;

#[derive(StructOpt, Debug)]
#[structopt(
    version = "1.4.1",
    about = "Productivity scripts for deployment",
    rename_all = "kebab-case"
)]
enum Drunx {
    Version(Version),
    Patch(Patch),
    Release(Release),
    CleanBranch(CleanBranch),
    Report(Report),
}

fn run() -> Result<(), Box<dyn Error>> {
    let cmd = Drunx::from_args();
    match cmd {
        Drunx::Version(version) => version.run()?,
        Drunx::Patch(patch) => patch.run()?,
        Drunx::Release(release) => release.run()?,
        Drunx::Report(report) => report.run()?,
        Drunx::CleanBranch(clean_branch) => clean_branch.run()?,
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Drunx Cmd Errors: {}", e);
        process::exit(1);
    }
}
