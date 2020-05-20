use regex::Regex;

pub fn find(contents: &str) -> Option<String> {
  let re = Regex::new(r#""version":\s"([0-9a-z\.]+)""#).unwrap();
  let capts = re.captures(contents);

  if capts.is_some() {
    Some(capts.unwrap()[1].to_string())
  } else {
    None
  }
}

#[cfg(test)]
mod find {
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
