mod aws;
mod git;
mod patch;
mod patch_utils;
mod release;
mod report;
mod version;
mod version_utils;

use aws::Aws;
use patch::Patch;
use release::Release;
use report::Report;
use std::error::Error;
use std::process;
use structopt::StructOpt;
use version::Version;

#[derive(StructOpt, Debug)]
#[structopt(
    version = "1.6.0",
    about = "Productivity commands",
    rename_all = "kebab-case"
)]
enum Drunx {
    Version(Version),
    Patch(Patch),
    Release(Release),
    Report(Report),
    Aws(Aws),
}

fn run() -> Result<(), Box<dyn Error>> {
    let cmd = Drunx::from_args();
    match cmd {
        Drunx::Version(version) => version.run()?,
        Drunx::Patch(patch) => patch.run()?,
        Drunx::Release(release) => release.run()?,
        Drunx::Report(report) => report.run()?,
        Drunx::Aws(aws) => aws.run()?,
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Drunx Cmd Errors: {}", e);
        process::exit(1);
    }
}
