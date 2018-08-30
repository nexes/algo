/// Binary search. Binary searches need a sorted array
///
/// The target is the value you want to search for in array. If the target is found
/// the index of that target will be returned, or None if the target was not found.
///
/// Be aware, with binary search, this may not be the index of the first occurance. If
/// you need the first occurance, use a linear search.
///
/// # Examples
/// ```
/// extern crate rs_algo;
/// use rs_algo::search::binary;
///
/// let s = vec![1, 2, 3, 5, 7, 8, 9, 23, 24, 56, 67];
/// assert_eq!(binary::index_of(23, &s), Some(7));
/// assert_eq!(binary::index_of(123, &s), None);
/// ```
pub fn index_of<T>(target: T, array: &Vec<T>) -> Option<usize>
where
  T: PartialOrd + Clone,
{
  if array.is_empty() {
    return None;
  }

  let mut base = 0;
  let mut size = array.len();

  while size > 1 {
    let half = size / 2;
    let mid = base + half;

    match array[mid] {
      ref value if *value == target => return Some(mid),
      ref value if *value < target => base = mid,
      ref value if *value > target => base = base,
      _ => return None,
    }

    size -= half;
  }

  None
}

/// Binary search. Binary searches need a sorted array
///
/// The target is the value you want to search for in array. If the target is found in the array
/// an optional of that value is returned. If the target wasn't found None will be returned.
///
/// # Examples
/// ```
/// extern crate rs_algo;
/// use rs_algo::search::binary;
///
/// let s = vec![1, 2, 3, 5, 7, 8, 9, 23, 24, 56, 67];
/// assert_eq!(binary::search(23, &s), Some(23));
/// assert_eq!(binary::search(123, &s), None);
/// ```
pub fn search<T>(target: T, array: &Vec<T>) -> Option<T>
where
  T: PartialOrd + Clone,
{
  if array.is_empty() {
    return None;
  }

  if array.len() == 1 {
    if array[0] == target {
      return Some(target);
    }
    return None;
  }

  let half = array.len() / 2;
  match array[half] {
    ref value if *value == target => Some(target),
    ref value if *value > target => search(target, &array[0..half].to_vec()),
    ref value if *value < target => search(target, &array[half..array.len()].to_vec()),
    _ => None,
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn binary_search() {
    use super::*;

    let array = vec![2, 3, 5, 6, 8, 9, 23, 54, 77, 78, 89, 90, 104];
    let str_array = vec![ "algorithm", "cake", "denver", "rust", "programming", "zebra", ];

    assert_eq!(search(77, &array), Some(77));
    assert_eq!(search("rust", &str_array), Some("rust"));
    assert_eq!(search("c++", &str_array), None);
  }

  #[test]
  fn binary_search_none() {
    use super::*;

    let array = vec![2, 3, 5, 6, 8, 9, 23, 54, 77, 78, 89, 90, 104];
    assert_eq!(search(999, &array), None);
  }

  #[test]
  fn binary_search_index() {
    use super::*;

    let array = vec![2, 3, 5, 6, 8, 9, 23, 54, 77, 78, 89, 89, 89, 90, 104];
    assert_eq!(index_of(9, &array), Some(5));
    // index 11 if correct, but not the first index of value 89. A side effect of binary search
    assert_eq!(index_of(89, &array), Some(11));
    assert_eq!(index_of(999, &array), None);

    let str_array = vec![ "algorithm", "cake", "denver", "rust", "programming", "zebra", ];
    assert_eq!(index_of("rust", &str_array), Some(3));
    assert_eq!(index_of("c++", &str_array), None);
  }
}
