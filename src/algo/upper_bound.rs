use cargo_snippet::snippet;

#[snippet]
fn upper_bound<T: PartialOrd>(array: &[T], k: T) -> usize {
    let mut ok = array.len() as i64;
    let mut ng = -1;

    while (ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        if array[mid as usize] <= k {
            ng = mid;
        } else {
            ok = mid;
        }
    }

    ok as usize
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn simple() {
        let v = [0, 1, 2, 2, 3, 4, 5];
        assert_eq!(upper_bound(&v, 2), 4);
    }

    #[test]
    fn simple_vector() {
        let v = vec![0, 1, 2, 2, 3, 4, 5];
        assert_eq!(upper_bound(&v, 2), 4);
    }

    #[test]
    fn simple_char() {
        let v = ['a', 'b', 'c', 'c', 'd', 'e', 'f'];
        assert_eq!(upper_bound(&v, 'c'), 4);
    }

    #[test]
    fn simple_max() {
        let v = vec![1, 2, 3];
        assert_eq!(upper_bound(&v, 5), 3);
    }

    #[test]
    fn simple_min() {
        let v = vec![4, 4, 5, 7, 8];
        assert_eq!(upper_bound(&v, -1), 0);
    }
}
