mod patch;
mod patch_utils;
mod version;
mod version_utils;

use patch::Patch;
use std::error::Error;
use std::process;
use structopt::StructOpt;
use version::Version;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Productivity scripts for deployment",
    rename_all = "kebab-case"
)]
enum Drunx {
    Version(Version),
    Patch(Patch),
}

fn run() -> Result<(), Box<dyn Error>> {
    let cmd = Drunx::from_args();
    match cmd {
        Drunx::Version(version) => version.run()?,
        Drunx::Patch(patch) => patch.run()?,
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Drunx Cmd Errors: {}", e);
        process::exit(1);
    }
}
