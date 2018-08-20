fn exchange<T>(array: &mut Vec<T>, j: usize, k: usize)
  where T: PartialOrd + Copy
{
  let temp = array[j];
  array[j] = array[k];
  array[k] = temp;
}

fn partition<T>(array: &mut Vec<T>, start_index: usize, end_index: usize) -> usize
  where T: PartialOrd + Copy
{
  let pivot = array[end_index];
  let mut j = start_index;

  for i in start_index..end_index {
    if array[i] <= pivot {
      exchange(array, i, j);
      j += 1;
    }
  }

  exchange(array, j, end_index);
  j
}

fn quick_sort<T>(array: &mut Vec<T>, start: usize, end: usize)
  where T: PartialOrd + Copy
{
  if start < end {
    let pivot = partition(array, start, end);

    if pivot > 0 {
      quick_sort(array, start, pivot - 1);
      quick_sort(array, pivot + 1, end);
    }
  }
}

pub fn sort<T>(array: &mut Vec<T>)
  where T: PartialOrd + Copy
{
  let len = array.len();
  if len == 0 {
    return;
  }

  quick_sort(array, 0, len - 1);
}


#[cfg(test)]
mod tests {
  #[test]
  fn quick_sort() {
    use super::*;

    let mut a = vec![3, 21, 9, 5, 0, -6, 8, 4, 4, 7];
    sort(&mut a);

    assert_eq!(a, vec![-6, 0, 3, 4, 4, 5, 7, 8, 9, 21]);
  }

  #[test]
  fn quick_sort_empty() {
    use super::*;

    let mut a: Vec<i32> = vec![];
    sort(&mut a);

    assert_eq!(a, vec![]);
  }

  #[test]
  fn quick_sort_single() {
    use super::*;

    let mut a = vec![3];
    sort(&mut a);

    assert_eq!(a, vec![3]);
  }

  #[test]
  fn quick_sort_str() {
    use super::*;

    let mut a = vec!["apple", "cake", "lemon", "zuccini", "berry", "black berry", "kit kat"];
    sort(&mut a);

    assert_eq!(a, vec!["apple", "berry", "black berry", "cake", "kit kat", "lemon", "zuccini"]);
  }
}