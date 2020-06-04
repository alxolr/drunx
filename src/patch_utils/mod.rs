pub fn compute_patch_letter(release: &str) -> String {
  let mut items = release.chars().collect::<Vec<char>>();
  let last = items.last().unwrap();

  if last.is_digit(10) {
    items.push('a');
  } else {
    if last == &'z' {
      let size = items.len();
      items[size - 1] = 'a';
      items.push('a');
    } else {
      let last_as_number = *last as u32;
      let next_letter = std::char::from_u32(last_as_number + 1).unwrap();

      let size = items.len();
      items[size - 1] = next_letter;
    }
  }

  items.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn should_create_a_version_if_first() {
    let release = "1.0.0";
    let expected = "1.0.0a".to_string();

    assert_eq!(compute_patch_letter(release), expected);
  }

  #[test]
  fn should_add_patch_version_b_if_a() {
    let release = "1.0.0a";
    let expected = "1.0.0b".to_string();

    assert_eq!(compute_patch_letter(release), expected);
  }

  #[test]
  fn for_the_case_of_last_letter_z_add_za() {
    let release = "1.0.0z";
    let expected = "1.0.0aa".to_string();

    assert_eq!(compute_patch_letter(release), expected);
  }
}
