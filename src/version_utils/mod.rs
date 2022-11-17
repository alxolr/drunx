use regex::Regex;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

pub fn extract_version(contents: &str) -> Option<String> {
    let regex_json = Regex::new(r#""version":\s?"([0-9a-z\.]+)""#).unwrap();
    let regex_txt = Regex::new(r#"Version\s([0-9a-z\.]+)"#).unwrap();

    let json_captures = regex_json.captures(contents);
    let txt_captures = regex_txt.captures(contents);

    if json_captures.is_some() {
        Some(json_captures.unwrap()[1].to_string())
    } else if txt_captures.is_some() {
        Some(txt_captures.unwrap()[1].to_string())
    } else {
        None
    }
}

pub fn get_version_from_file(file_path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open(file_path.as_path())?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let version = extract_version(&contents);

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

    let version = extract_version(&contents);
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
    use super::extract_version;

    #[test]
    fn extracts_the_version_if_exits() {
        let content = r#"\
    {
        "name": "some_name",
        "version": "1.0.0",
    }"#;

        assert_eq!(extract_version(content).unwrap(), "1.0.0".to_string());
    }

    #[test]
    fn extracts_the_version_including_letters() {
        let content = r#"\
    {
        "name": "some_name",
        "version": "1.0.0x",
    }"#;

        assert_eq!(extract_version(content).unwrap(), "1.0.0x".to_string());
    }

    #[test]
    fn return_none_version_not_found() {
        let content = r#"\
    {
        "name": "some_name",
    }"#;

        assert_eq!(extract_version(content), None);
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

        assert_eq!(extract_version(content), Some(String::from("1.0.0x")));
    }
}
