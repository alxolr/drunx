use regex::Regex;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

pub fn find(contents: &str) -> Option<String> {
    let re = Regex::new(r#""version":\s?"([0-9a-z\.]+)""#).unwrap();
    let capts = re.captures(contents);

    if capts.is_some() {
        Some(capts.unwrap()[1].to_string())
    } else {
        None
    }
}

pub fn get_version_from_file(file_path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open(file_path.as_path())?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let version = find(&contents);

    match version {
        Some(version) => Ok(version),
        None => panic!("Couldn't find the version in the file"),
    }
}

pub fn change_version(file_path: &PathBuf, next_version: &str) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open(file_path.as_path())?;

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let version = find(&contents);
    if version.is_some() {
        contents = contents.replacen(&version.unwrap(), next_version, 2);
    } else {
        println!("Couldn't indentify the version");
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path.as_path())?;

    file.write_all(contents.as_bytes())?;
    file.sync_all()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::find;

    #[test]
    fn extracts_the_version_if_exits() {
        let content = r#"\
    {
        "name": "some_name",
        "version": "1.0.0",
    }"#;

        assert_eq!(find(content).unwrap(), "1.0.0".to_string());
    }

    #[test]
    fn extracts_the_version_including_letters() {
        let content = r#"\
    {
        "name": "some_name",
        "version": "1.0.0x",
    }"#;

        assert_eq!(find(content).unwrap(), "1.0.0x".to_string());
    }

    #[test]
    fn return_none_version_not_found() {
        let content = r#"\
    {
        "name": "some_name",
    }"#;

        assert_eq!(find(content), None);
    }

    #[test]
    fn extract_only_first_layer_of_version() {
        let content = r#"\
    {
        "name": "some_name",
        "version": "1.0.0x",
        "docs": {
          "version": "1.0.3"
        }
    }"#;

        assert_eq!(find(content), Some(String::from("1.0.0x")));
    }
}
