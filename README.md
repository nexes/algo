## Algo

A small crate of commonly used sorting algorithms for any generic type that implements PartialOrd and copy.

The crate can be found here: [Crate](https://crates.io/crates/rs_algo)

### Usage
```rust
[dependencies]
rs_algo = "0.1"
```

### Example
```rust
use rs_algo::sort::*;


fn main() {
  let mut a = vec![117, 1, 3, 99, 10, 7, 7, 2, 11, -5, 4, 9, 32, 1, 3, 99, 10, 7];
  let mut b = vec![117, 1, 3, 99, 10, 7, 7, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7];
  let mut c = vec![117, 1, 3, 99, 10, 7, 7, 2, 11, -5, 4, 9, 817, 1, 3, 99, 10, 7];
  let mut d = vec!["apple", "cake", "lemon", "zuccini", "berry", "black berry", "kit kat"];

  let time = merge::sort_with_time(&mut a);
  println!("merge sorted: time {:?}", time);

  let time = quick::sort_with_time(&mut b);
  println!("quick sorted: time {:?}", time);

  let time = insertion::sort_with_time(&mut c);
  println!("insertion sorted: time {:?}", time);

  let time = bubble::sort_with_time(&mut d);
  println!("bubble sorted: time {:?}", time);
}
```

## License
MIT