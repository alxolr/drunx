extern crate clap;
extern crate regex;

#[cfg(test)]
extern crate assert_cmd;
#[cfg(test)]
extern crate predicates;

use clap::{App, Arg, ArgMatches, SubCommand};
use regex::Regex;
use std::error;

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn new<'a, 'b>() -> Result<App<'a, 'b>> {
    Ok(SubCommand::with_name("patch")
        .about("Increment the patch letter in the target project")
        .arg(
            Arg::with_name("dry-run")
                .short("d")
                .long("dry-run")
                .help("Show the steps to be executed without executing them"),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .help("Provide the project directory, default current directory"),
        )
        .arg(Arg::with_name("sprint").required(true)))
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    println!("What is this shit {:#?}", matches);
    Ok(())
}

fn extract_package_version<'a>(contents: &'a str) -> Result<String> {
    let re = Regex::new(r#""version":\s"([1-9\.]+)""#)?;
    let captures = re.captures(contents);

    match captures {
        Some(capture) => return Ok((&capture[1]).to_string()),
        None => panic!("Expecting to have version"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;

    #[test]
    fn sprint_number_should_be_provided() -> Result<()> {
        let mut cmd = Command::cargo_bin("drunx")?;
        cmd.arg("patch")
            .assert()
            .failure()
            .stderr(predicates::str::contains(
                "error: The following required arguments were not provided:\n    <sprint>",
            ));

        Ok(())
    }

    #[test]
    fn extract_version_out_of_file() {
        let contents = r#"\
{
    "name": "some_name",
    "version": "4.4.3",
}"#;
        let expected = "4.4.3".to_string();
        let result = extract_package_version(contents).unwrap();
        assert_eq!(result, expected);
    }
}
