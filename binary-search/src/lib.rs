use std::cmp::Ordering;

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> Option<usize>;
    fn upper_bound(&self, x: &T) -> Option<usize>;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> Option<usize> {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                },
                Ordering::Greater | Ordering::Equal => {
                    high = mid;
                },
            }
        }
        if low < self.len() {
            Some(low)
        } else {
            None
        }
    }
    fn upper_bound(&self, x: &T) -> Option<usize> {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                },
                Ordering::Greater => {
                    high = mid;
                },
            }
        }
        if low < self.len() {
            Some(low)
        } else {
            None
        }
    }
}#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let vec = vec![1, 2, 4, 6, 7, 12, 54, 60];

        assert_eq!(vec.lower_bound(&4), Some(2));
        assert_eq!(vec.upper_bound(&4), Some(3));
        assert_eq!(vec.lower_bound(&5), Some(3));
        assert_eq!(vec.upper_bound(&5), Some(3));
        assert_eq!(vec.lower_bound(&60), Some(7));
        assert_eq!(vec.upper_bound(&60), None);
    }
}