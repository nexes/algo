use std::time::{ SystemTime, Duration };

pub fn sort<T>(array: &mut Vec<T>)
  where T: PartialOrd + Copy
{
  let len = array.len();
  if len < 2 {
    return;
  }

  for i in 1..len {
    let key = array[i];
    let mut j = i;

    while j > 0 && array[j - 1] > key {
      array[j] = array[j - 1];
      j -= 1;
    }

    array[j] = key;
  }
}

pub fn sort_with_time<T>(a: &mut Vec<T>) -> Duration
  where T: PartialOrd + Copy
{
  let duration: Duration;
  let timer = SystemTime::now();
  sort(a);

  match timer.elapsed() {
    Ok(d) => {
      duration = d;
    },
    Err(e) => {
      duration = Duration::new(0, 0);
      println!("merge sort: error getting duration, {:?}", e)
    }
  }

  duration
}

#[cfg(test)]
mod tests {
  #[test]
  fn insertion_sort() {
    use super::*;

    let mut a = vec![3, 21, 9, 5, 0, -6, 8, 4, 4, 7];
    sort(&mut a);

    assert_eq!(a, vec![-6, 0, 3, 4, 4, 5, 7, 8, 9, 21]);
  }

  #[test]
  fn insertion_sort_empty() {
    use super::*;

    let mut a: Vec<i32> = vec![];
    sort(&mut a);

    assert_eq!(a, vec![]);
  }

  #[test]
  fn insertion_sort_str() {
    use super::*;

    let mut a = vec!["apple", "cake", "lemon", "zuccini", "berry", "black berry", "kit kat"];
    sort(&mut a);

    assert_eq!(a, vec!["apple", "berry", "black berry", "cake", "kit kat", "lemon", "zuccini"]);
  }
}