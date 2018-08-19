fn merge<T>(left: &Vec<T>,  right: &Vec<T>, array: &mut Vec<T>)
  where T: PartialOrd + Copy
{
  let mut left_index = 0;
  let mut right_index = 0;
  let mut index = 0;

  while left_index < left.len() && right_index < right.len() {
    if left[left_index] <= right[right_index] {
      array[index] = left[left_index];
      left_index += 1;

    } else {
      array[index] = right[right_index];
      right_index += 1;
    }

    index += 1;
  }

  while left_index < left.len() {
    array[index] = left[left_index];
    left_index += 1;
    index += 1;
  }
  while right_index < right.len() {
    array[index] = right[right_index];
    right_index += 1;
    index += 1;
  }
}

fn divide_list<T>(array: &mut Vec<T>)
  where T: PartialOrd + Copy
{
  let len = array.len();

  if len > 1 {
    let mid =  len / 2;
    let mut left: Vec<T> = Vec::with_capacity(mid);
    let mut right: Vec<T> = Vec::with_capacity(len - mid);

    for i in 0..mid {
      left.push(array[i]);
    }
    for i in mid..len {
      right.push(array[i]);
    }

    divide_list(&mut left);
    divide_list(&mut right);

    merge(&left, &right, array);
  }
}

pub fn sort<T>(a: &mut Vec<T>)
  where T: PartialOrd + Copy
{
  let size: usize = a.len();
  if size < 2 {
    return;
  }

  divide_list(a);
}


#[cfg(test)]
mod tests {
  #[test]
  fn merge_test() {
    use super::*;

    let mut a = vec![17, 1, 3, 99, 10, 7, 2, 11, -5, 4, 9, 8];
    sort(&mut a);

    assert_eq!(a, vec![-5, 1, 2, 3, 4, 7, 8, 9, 10, 11, 17, 99]);
  }

  #[test]
  fn merge_single_array() {
    use super::*;

    let mut a: Vec<i32> = vec![23];
    sort(&mut a);

    assert_eq!(a, vec![23]);
  }

  #[test]
  fn merge_empty_array() {
    use super::*;

    let mut a: Vec<u8> = vec![];
    sort(&mut a);

    assert_eq!(a, vec![]);
  }

  #[test]
  fn merge_str_array() {
    use super::*;

    let mut a: Vec<&str> = vec!["December", "May", "June", "April", "July", "Batman"];
    sort(&mut a);

    assert_eq!(a, vec!["April", "Batman", "December", "July", "June", "May"]);
  }
}