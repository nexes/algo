use std::time::{Duration, SystemTime};

fn exchange<T>(array: &mut Vec<T>, i: usize, j: usize)
where
    T: PartialOrd + Copy,
{
    let temp = array[i];
    array[i] = array[j];
    array[j] = temp;
}

/// Return a sorted array, the original will not be chagned
///
/// # Examples
/// ```
/// use rs_algo::sort::bubble;
///
/// let mut a = vec![3, 2, -8, 34, 2, 8];
/// let sorted = bubble::sort(&mut a);
///
/// assert_eq!(sorted, vec![-8, 2, 2, 3, 8, 34]);
/// assert_eq!(a, vec![3, 2, -8, 34, 2, 8]);
/// ```
pub fn sort<T>(array: &Vec<T>) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let mut sorted = array.clone();
    sort_mut(&mut sorted);
    sorted
}

/// Sort the given vector with bubble sort, the vector will be sorted in place
///
/// # Examples
/// ```
/// use rs_algo::sort::bubble;
///
/// let mut a = vec![3, 2, -8, 34, 2, 8];
/// bubble::sort_mut(&mut a);
///
/// assert_eq!(a, vec![-8, 2, 2, 3, 8, 34]);
/// ```
pub fn sort_mut<T>(array: &mut Vec<T>)
where
    T: PartialOrd + Copy,
{
    let mut exchanged: bool;
    let mut pass = 1;
    let len = array.len();

    if len < 2 {
        return;
    }

    loop {
        exchanged = false;

        for i in 0..len - pass {
            if array[i] > array[i + 1] {
                exchange(array, i, i + 1);
                exchanged = true;
            }
        }

        pass += 1;
        if !exchanged {
            break;
        }
    }
}

/// Sort the given vector with insertion sort, the vector will be sorted, this will return the time duration it took to sort
///
/// # Examples
/// ```
/// use rs_algo::sort::bubble;
///
/// let mut a = vec![3, 2, -8, 34, 2, 8];
/// bubble::sort_with_time(&mut a);
///
/// assert_eq!(a, vec![-8, 2, 2, 3, 8, 34]);
/// ```
pub fn sort_with_time<T>(a: &mut Vec<T>) -> Duration
where
    T: PartialOrd + Copy,
{
    let duration: Duration;
    let timer = SystemTime::now();
    sort_mut(a);

    match timer.elapsed() {
        Ok(d) => {
            duration = d;
        }
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
    fn bubble_sort() {
        use super::*;

        let a = vec![3, 21, 9, 5, 0, -6, 8, 4, 4, 7];
        let sorted = sort(&a);

        assert_eq!(sorted, vec![-6, 0, 3, 4, 4, 5, 7, 8, 9, 21]);
        assert_eq!(a, vec![3, 21, 9, 5, 0, -6, 8, 4, 4, 7]);
    }

    #[test]
    fn bubble_sort_mut() {
        use super::*;

        let mut a = vec![3, 21, 9, 5, 0, -6, 8, 4, 4, 7];
        sort_mut(&mut a);

        assert_eq!(a, vec![-6, 0, 3, 4, 4, 5, 7, 8, 9, 21]);
    }

    #[test]
    fn bubble_sort_empty() {
        use super::*;

        let mut a: Vec<i32> = vec![];
        sort_mut(&mut a);

        assert_eq!(a, vec![]);
    }

    #[test]
    fn bubble_sort_str_mut() {
        use super::*;

        let mut a = vec![
            "apple",
            "cake",
            "lemon",
            "zuccini",
            "berry",
            "black berry",
            "kit kat",
        ];
        sort_mut(&mut a);

        assert_eq!(
            a,
            vec![
                "apple",
                "berry",
                "black berry",
                "cake",
                "kit kat",
                "lemon",
                "zuccini"
            ]
        );
    }
}
