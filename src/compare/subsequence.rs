use std::cmp::max;
use std::fmt::{Display, Error, Formatter};

/// An objecft that will hold the given two strings and their grid so you can later ask for the longest common subsequence
/// without having to re-create the 2d grid
///
/// # Examples
/// ```
/// use rs_algo::compare::LCSubsequence;
///
/// let lcs = LCSubsequence::new_subsequence("abttac".to_string(), "taccba".to_string());
///
/// println!("length of the lcs is {}", lcs);
/// assert_eq!(lcs.get_longest_subsequence(), Some("aba".to_string()));
/// ```
#[derive(Debug)]
pub struct LCSubsequence {
    /// the length of the found longest common subsequence
    pub subsequence_len: u32,
    grid: Vec<Vec<u32>>,
    left_str: String,
    right_str: String,
}

impl Display for LCSubsequence {
    /// Print the size of the longest subsequence
    /// # Examples
    /// ```
    /// use rs_algo::compare::LCSubsequence;
    ///
    /// let lcs = LCSubsequence::new_subsequence("abttac".to_string(), "taccba".to_string());
    ///
    /// println!("length of the lcs is {}", lcs);
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.subsequence_len)
    }
}

impl LCSubsequence {
    /// This will return a new LCS object. The subsequence grid will be created and stored, the longest subsequence length
    /// will be known at this time.
    pub fn new_subsequence<T>(left: T, right: T) -> Self
    where
        T: AsRef<str>,
    {
        let left_chars: Vec<char> = left.as_ref().chars().collect();
        let right_chars: Vec<char> = right.as_ref().chars().collect();
        let left_len = left_chars.len();
        let right_len = right_chars.len();

        let mut l = LCSubsequence {
            subsequence_len: 0,
            grid: vec![vec![0; right_len + 1]; left_len + 1],
            left_str: left.as_ref().to_string(),
            right_str: right.as_ref().to_string(),
        };

        for i in 1..left_len + 1 {
            for j in 1..right_len + 1 {
                if left_chars[i - 1] == right_chars[j - 1] {
                    l.grid[i][j] = l.grid[i - 1][j - 1] + 1;
                } else {
                    l.grid[i][j] = max(l.grid[i][j - 1], l.grid[i - 1][j]);
                }
            }
        }

        l.subsequence_len = l.grid[left_len][right_len];
        l
    }

    /// Return the longest subsequence string as an option.
    /// If there was no longest common subsequence found during the new_subsequence call, this will return None,
    /// otherwise the lcs as a string will be returned.
    pub fn get_longest_subsequence(&self) -> Option<String> {
        if self.subsequence_len == 0 {
            return None;
        }

        let mut i = self.grid.len() - 1;
        let mut j = self.grid[0].len() - 1;
        let mut output: Vec<u16> = Vec::new();
        let left_chars: Vec<char> = self.left_str.chars().collect();
        let right_chars: Vec<char> = self.right_str.chars().collect();

        while i > 0 && j > 0 {
            if left_chars[i - 1] == right_chars[j - 1] {
                output.push(left_chars[i - 1] as u16);
                i -= 1;
                j -= 1;
            } else {
                if self.grid[i - 1][j] >= self.grid[i][j - 1] {
                    i -= 1;
                } else {
                    j -= 1;
                }
            }
        }

        output.reverse();
        Some(String::from_utf16(output.as_ref()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn lc_subsequence_test() {
        use super::*;

        let a = String::from("abbtaabtca");
        let b = "ax3taaca";
        let lcs = LCSubsequence::new_subsequence(a, b.to_string());

        assert_eq!(lcs.subsequence_len, 6);
        assert_eq!(lcs.get_longest_subsequence(), Some("ataaca".to_string()));
    }

    #[test]
    fn lc_subsequence_one_test() {
        use super::*;

        let a = String::from("abbtaabtca");
        let b = "x";
        let lcs = LCSubsequence::new_subsequence(a, b.to_string());

        assert_eq!(lcs.subsequence_len, 0);
        assert_eq!(lcs.get_longest_subsequence(), None);
    }

    #[test]
    fn lc_subsequence_empty_test() {
        use super::*;

        let a = String::from("");
        let b = "";
        let lcs = LCSubsequence::new_subsequence(a, b.to_string());

        assert_eq!(lcs.subsequence_len, 0);
        assert_eq!(lcs.get_longest_subsequence(), None);
    }
}
