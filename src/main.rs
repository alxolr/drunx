mod version_utils;
use structopt::StructOpt;

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::process;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Productivity scripts for deployment",
    rename_all = "kebab-case"
)]
enum Drunx {
    Version {
        #[structopt(help = "Sprint version tag")]
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
    },
}

fn change_version_in_file(newVersion: &str, filePath: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open(filePath.as_path())?;

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let version = version_utils::find(&contents);
    if version.is_some() {
        contents = contents.replace(&version.unwrap(), newVersion);
    }

    let mut file = OpenOptions::new().write(true).open(filePath.as_path())?;

    file.write_all(contents.as_bytes())?;
    file.sync_all()?;

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let Drunx::Version {
        release,
        path,
        dry_run,
    } = Drunx::from_args();

    let package_json_path = PathBuf::from(&path).join("package.json");
    let package_lock_json_path = PathBuf::from(&path).join("package-lock.json");

    if dry_run {
        println!(
            "Changing version to in package.json and package-lock.json {}",
            release
        );
    } else {
        change_version_in_file(&release, &package_json_path)?;
        change_version_in_file(&release, &package_lock_json_path)?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Drunx Cmd Errors: {}", e);
        process::exit(1);
    }
}
