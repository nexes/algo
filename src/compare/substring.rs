use std::fmt::{ Display, Formatter, Error };


/// An objecft that will hold the given two strings and their grid so you can later ask for the longest common substring
/// without having to re-create the 2d grid
///
/// # Examples
/// ```
/// use rs_algo::compare::LCSubstring;
///
/// let lcs = LCSubstring::new_substring("sunny today outside".to_string(), "today is cold".to_string());
///
/// println!("length of the lc substring is {}", lcs);
/// assert_eq!(lcs.get_longest_substring(), Some("today ".to_string()));
/// ```
#[derive(Debug)]
pub struct LCSubstring {
  /// the length of the found longest common substring
  pub substring_len: u32,

  grid: Vec<Vec<u32>>,
  max_substring_index: (usize, usize),
  left_str: String,
  right_str: String,
}

impl Display for LCSubstring {
  /// Print the size of the longest substring
  /// # Examples
  /// # Examples
  /// ```
  /// use rs_algo::compare::LCSubstring;
  ///
  /// let lcs = LCSubstring::new_substring("sunny today outside".to_string(), "today is cold".to_string());
  ///
  /// println!("length of the lc substring is {}", lcs);
  /// ```
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{}", self.substring_len)
  }
}

impl LCSubstring {
  /// This will return a new LCSubstring object. The substring grid will be created and stored, the longest substring length
  /// will be known at this time.
  pub fn new_substring<T>(left: T, right: T) -> Self
    where T: AsRef<str>
  {
    let left_chars: Vec<char> = left.as_ref().chars().collect();
    let right_chars: Vec<char> = right.as_ref().chars().collect();
    let left_len = left_chars.len();
    let right_len = right_chars.len();

    let mut sub_string = LCSubstring {
      substring_len: 0,
      grid: vec![vec![0; right_len + 1]; left_len + 1],
      max_substring_index: (0, 0),
      left_str: left.as_ref().to_string(),
      right_str: right.as_ref().to_string(),
    };

    for i in 1..left_len + 1 {
      for j in 1..right_len + 1 {
        if left_chars[i - 1] == right_chars[j - 1] {
          sub_string.grid[i][j] = sub_string.grid[i - 1][j - 1] + 1;

          if sub_string.substring_len < sub_string.grid[i][j] {
            sub_string.max_substring_index = (i, j);
            sub_string.substring_len = sub_string.grid[i][j];
          }
        }
      }
    }

    sub_string
  }

  /// Return the longest substring as an option.
  /// If there was no longest common substring found during the new_substring call, this will return None,
  /// otherwise the lc substring as a string will be returned.
  pub fn get_longest_substring(&self) -> Option<String> {
    if self.substring_len == 0 {
      return None;
    }

    let mut lcs: Vec<u16> = Vec::new();
    let left_str: Vec<char> = self.left_str.chars().collect();
    let offset: usize = self.max_substring_index.0;

    for i in 0..self.substring_len {
      let index = (offset - 1) - i as usize;
      lcs.push(left_str[index as usize] as u16);
    }

    lcs.reverse();
    Some(String::from_utf16(lcs.as_ref()).unwrap())
  }
}


#[cfg(test)]
mod tests {
  #[test]
  fn lc_substring_test() {
    use super::*;

    let a = String::from("Why hello world you world hello");
    let b = String::from("subby world Why hello red car go fast");
    let lcs = LCSubstring::new_substring(a, b);

    assert_eq!(lcs.substring_len, 10);
  }

  #[test]
  fn lc_substring_empty_test() {
    use super::*;

    let a = String::from("Why hello world you world hello");
    let b = String::from("");
    let lcs = LCSubstring::new_substring(a, b);

    assert_eq!(lcs.substring_len, 0);
    assert_eq!(lcs.get_longest_substring(), None);
  }

  #[test]
  fn lc_substring_get_string_test() {
    use super::*;

    let a = String::from("!!!Why hello world you world hello");
    let b = String::from("subby world Why hello red car go fast");
    let lcs = LCSubstring::new_substring(a, b);

    assert_eq!(lcs.get_longest_substring(), Some("Why hello ".to_string()));
  }
}
