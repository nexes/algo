fn exchange<T>(array: &mut Vec<T>, j: usize, k: usize)
where
    T: PartialOrd + Copy,
{
    let temp = array[j];
    array[j] = array[k];
    array[k] = temp;
}

fn partition<T>(array: &mut Vec<T>, start_index: usize, end_index: usize) -> usize
where
    T: PartialOrd + Copy,
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
where
    T: PartialOrd + Copy,
{
    if start < end {
        let pivot = partition(array, start, end);

        if pivot > 0 {
            quick_sort(array, start, pivot - 1);
            quick_sort(array, pivot + 1, end);
        }
    }
}

/// Returns a sort vector with quick sort, the original will not be changed
///
/// # Examples
/// ```
/// use rs_algo::sort::quick;
///
/// let mut a = vec![3, 2, -8, 34, 2, 8];
/// let sorted = quick::sort(&mut a);
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

/// Sort the given vector with quick sort, the vector will be sorted
///
/// # Examples
/// ```
/// use rs_algo::sort::quick;
///
/// let mut a = vec![3, 2, -8, 34, 2, 8];
/// quick::sort_mut(&mut a);
///
/// assert_eq!(a, vec![-8, 2, 2, 3, 8, 34]);
/// ```
pub fn sort_mut<T>(array: &mut Vec<T>)
where
    T: PartialOrd + Copy,
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

        let a = vec![3, 21, 9, 5, 0, -6, 8, 4, 4, 7];
        let sorted = sort(&a);

        assert_eq!(sorted, vec![-6, 0, 3, 4, 4, 5, 7, 8, 9, 21]);
        assert_eq!(a, vec![3, 21, 9, 5, 0, -6, 8, 4, 4, 7]);
    }

    #[test]
    fn quick_sort_mut() {
        use super::*;

        let mut a = vec![3, 21, 9, 5, 0, -6, 8, 4, 4, 7];
        sort_mut(&mut a);

        assert_eq!(a, vec![-6, 0, 3, 4, 4, 5, 7, 8, 9, 21]);
    }

    #[test]
    fn quick_sort_empty() {
        use super::*;

        let mut a: Vec<i32> = vec![];
        sort_mut(&mut a);

        assert_eq!(a, vec![]);
    }

    #[test]
    fn quick_sort_single() {
        use super::*;

        let mut a = vec![3];
        sort_mut(&mut a);

        assert_eq!(a, vec![3]);
    }

    #[test]
    fn quick_sort_str() {
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
