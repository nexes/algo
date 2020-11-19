/// Return a sorted array, the original will not be chagned
///
/// # Examples
/// ```
/// use rs_algo::sort::insertion;
///
/// let mut a = vec![3, 2, -8, 34, 2, 8];
/// let sorted = insertion::sort(&mut a);
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

/// Sort the given vector with insertion sort, the vector will be sorted
///
/// # Examples
/// ```
/// use rs_algo::sort::insertion;
///
/// let mut a = vec![3, 2, -8, 34, 2, 8];
/// insertion::sort_mut(&mut a);
///
/// assert_eq!(a, vec![-8, 2, 2, 3, 8, 34]);
/// ```
pub fn sort_mut<T>(array: &mut Vec<T>)
where
    T: PartialOrd + Copy,
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

#[cfg(test)]
mod tests {
    #[test]
    fn insertion_sort() {
        use super::*;

        let a = vec![3, 21, 9, 5, 0, -6, 8, 4, 4, 7];
        let sorted = sort(&a);

        assert_eq!(sorted, vec![-6, 0, 3, 4, 4, 5, 7, 8, 9, 21]);
        assert_eq!(a, vec![3, 21, 9, 5, 0, -6, 8, 4, 4, 7]);
    }

    #[test]
    fn insertion_sort_mut() {
        use super::*;

        let mut a = vec![3, 21, 9, 5, 0, -6, 8, 4, 4, 7];
        sort_mut(&mut a);

        assert_eq!(a, vec![-6, 0, 3, 4, 4, 5, 7, 8, 9, 21]);
    }

    #[test]
    fn insertion_sort_empty() {
        use super::*;

        let mut a: Vec<i32> = vec![];
        sort_mut(&mut a);

        assert_eq!(a, vec![]);
    }

    #[test]
    fn insertion_sort_str() {
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
