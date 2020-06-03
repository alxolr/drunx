mod patch_utils;
mod version;
mod version_utils;

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
}

fn run() -> Result<(), Box<dyn Error>> {
    let Drunx::Version(version) = Drunx::from_args();
    version.run()?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Drunx Cmd Errors: {}", e);
        process::exit(1);
    }
}
