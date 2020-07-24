pub fn compute_patch_letter(release: &str) -> String {
  let version = release
    .chars()
    .filter(|c| !c.is_alphabetic())
    .collect::<String>();

  let mut patch_part: Vec<char> = release
    .chars()
    .filter(|c| c.is_alphabetic())
    .collect::<Vec<char>>();

  if patch_part.is_empty() {
    patch_part.push('a');
  } else {
    let mut iter = 0;

    loop {
      if patch_part.is_empty() {
        iter += 1;
        while iter > 0 {
          patch_part.push('a');
          iter -= 1;
        }

        break;
      }

      let last = patch_part.pop().unwrap();
      iter += 1;

      if last == 'z' {
        continue;
      } else {
        let char_as_number = last as u32;
        let next_letter = std::char::from_u32(char_as_number + 1).unwrap();
        patch_part.push(next_letter);
        while iter > 1 {
          patch_part.push('a');
          iter -= 1;
        }

        break;
      }
    }
  }

  format!("{}{}", version, patch_part.iter().collect::<String>())
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

  #[test]
  fn for_the_case_of_last_letter_aa_add_ab() {
    let release = "1.0.0aa";
    let expected = "1.0.0ab".to_string();

    assert_eq!(compute_patch_letter(release), expected);
  }

  #[test]
  fn for_the_case_of_last_letter_az_add_ba() {
    let release = "1.0.0az";
    let expected = "1.0.0ba".to_string();

    assert_eq!(compute_patch_letter(release), expected);
  }

  #[test]
  fn for_the_case_of_last_letter_zz_add_aaa() {
    let release = "1.0.0zz";
    let expected = "1.0.0aaa".to_string();

    assert_eq!(compute_patch_letter(release), expected);
  }
}
