## Algo

A small crate of commonly used sorting algorithms for any generic type that implements PartialOrd and Copy.

The crate can be found here: [Crate](https://crates.io/crates/rs_algo)

### Sorting
* merge sort
* quick sort
* insertion sort
* bubble sort

### Compare
* Find the longest common subsequence of two strings
* Find the longest common substring of two strings

### Search
* Find a value or it's index through binary search

### Math
* Greatest common divisor (Euclid's algorithm)
* Find the factors of a number
* Check if two numbers are relatively prime

## Usage
```rust
[dependencies]
rs_algo = "0.1"
```

## Example
```rust
extern crate rs_algo;

use rs_algo::math;
use rs_algo::sort::*;
use rs_algo::search::binary;
use rs_algo::compare::{LCSubsequence, LCSubstring};


fn main() {
  let mut a = vec![117, 1, 3, 99, 10, 7, 17, 2, 11, -6, 4, 9, 10, 7, 2, 11, -5, 4, 9, 7, 2, 11, -5, 4, 9, 8];
  let mut b = vec![117, 1, 3, 99, 10, 7, 17, 2, 11, -6, 4, 9, 10, 7, 2, 11, -5, 4, 9, 7, 2, 11, -5, 4, 9, 8];
  let mut c = vec![117, 1, 3, 99, 10, 7, 17, 2, 11, -6, 4, 9, 10, 7, 2, 11, -5, 4, 9, 7, 2, 11, -5, 4, 9, 8];
  let mut d = vec![117, 1, 3, 99, 10, 7, 17, 2, 11, -6, 4, 9, 10, 7, 2, 11, -5, 4, 9, 7, 2, 11, -5, 4, 9, 8];

  // This will sort the vector passed in, changing the original vector order
  merge::sort(&mut a);
  quick::sort(&mut b);
  insertion::sort(&mut c);
  bubble::sort(&mut d);

  // Or if want the time taken you can use sort_with_time
  //
  // let time = merge::sort_with_time(&mut a);
  // let time = quick::sort_with_time(&mut b);
  // let time = insertion::sort_with_time(&mut c);
  // let time = bubble::sort_with_time(&mut d);

  // get a new longest common sequence object
  let sequence = LCSubsequence::new_subsequence("leighxxxft".to_string(), "right".to_string());
  assert_eq!(sequence.subsequence_len, 4);
  assert_eq!(sequence.get_longest_subsequence(), Some("ight".to_string()));

  // get a new longest common substring
  let substring = LCSubstring::new_substring("!!!!Hello WorldXXXXX".to_string(), "XX Hello World@cvcvcvc".to_string());
  assert_eq!(substring.substring_len, 11);
  assert_eq!(substring.get_longest_substring(), Some("Hello World".to_string()));

  // do a binary search on array 'a' to see if value 99 is in the array
  match binary::search(99, &a) {
    Some(value) => println!("our array has value {}", value),
    None => println!("our array dosen't have value 99"),
  }

  // do a binary search on array 'a' to get the index. Note: with binary search, this may not be the first occurance
  match binary::index_of(99, &a) {
    Some(index) => println!("index of 99 is {}", index),
    None => println!("no index of 99, guess it's not in there"),
  }

  // common math functions
  let divisor = math::gcd(30, 21);
  let factor = math::factors(9124);

  assert_eq!(Ok(3), divisor);
  assert_eq!(Some(vec![2, 4, 2281, 4562]), factor);
}
```

## License
MIT
