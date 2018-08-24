extern crate rs_algo;

use rs_algo::sort::*;
use rs_algo::compare::{LCSubsequence, LCSubstring};


// test
fn main() {
  let mut a = vec![117, 1, 3, 99, 10, 7, 7, 2, 11, -5, 4, 9, 32, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 32, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 2, 11, -5, 4, 9, 32, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 8];
  let mut b = vec![117, 1, 3, 99, 10, 7, 7, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 8];
  let mut c = vec![117, 1, 3, 99, 10, 7, 7, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 8];
  let mut d = vec![117, 1, 3, 99, 10, 7, 7, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 8];

  let time = merge::sort_with_time(&mut a);
  println!("merge sorted: time {:?}", time);

  let time = quick::sort_with_time(&mut b);
  println!("quick sorted: time {:?}", time);

  let time = insertion::sort_with_time(&mut c);
  println!("insertion sorted: time {:?}", time);

  let time = bubble::sort_with_time(&mut d);
  println!("bubble sorted: time {:?}", time);

  // get a new longest common sequence object
  let lcs = LCSubsequence::new_subsequence("leighxxxft".to_string(), "right".to_string());
  println!("lcs is = {}", lcs.subsequence_len);
  println!("longest common subsequence = {}", lcs.get_longest_subsequence().expect("no lcs found"));
}