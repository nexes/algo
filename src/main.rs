use rs_algo::compare::{LCSubsequence, LCSubstring};
use rs_algo::math;
use rs_algo::search::binary;
use rs_algo::sort::*;

// test
fn main() {
    let mut a = vec![
        117, 1, 3, 99, 10, 7, 7, 2, 11, -5, 4, 9, 32, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 32, 1, 3,
        99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 2, 11, -5, 4, 9, 32, 1, 3, 99, 10, 7, 2, 11, -5,
        4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 8,
    ];
    let mut b = vec![
        117, 1, 3, 99, 10, 7, 7, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 817, 1, 3,
        99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5,
        4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 8,
    ];
    let mut c = vec![
        117, 1, 3, 99, 10, 7, 7, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 817, 1, 3,
        99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5,
        4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 8,
    ];
    let mut d = vec![
        117, 1, 3, 99, 10, 7, 7, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 817, 1, 3,
        99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5,
        4, 9, 817, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 87, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 8,
    ];

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
    let substring = LCSubstring::new_substring(
        "!!!!Hello WorldXXXXX".to_string(),
        "XXX   Hello World@cvcvcvc".to_string(),
    );
    assert_eq!(substring.substring_len, 11);
    assert_eq!(
        substring.get_longest_substring(),
        Some("Hello World".to_string())
    );

    // search our array. Binary search needs the array to already be sorted
    match binary::search(817, &a) {
        Some(value) => println!("our array has value {}", value),
        None => println!("our array dosen't have value 817"),
    }

    match binary::index_of(817, &a) {
        Some(index) => println!("index of 817 is {}", index),
        None => println!("no index of 817, guess it's not in there"),
    }

    // common math functions
    let divisor = math::gcd(30, 21);
    assert_eq!(Ok(3), divisor);

    let factor = math::factors(9124);
    assert_eq!(Some(vec![2, 4, 2281, 4562]), factor);
}
