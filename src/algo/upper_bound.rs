use cargo_snippet::snippet;

#[snippet]
fn upper_bound(array: &Vec<u64>, k: u64) -> u64 {
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

    ok as u64
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn simple() {
        let v = vec![0, 1, 2, 2, 3, 4, 5];
        assert_eq!(upper_bound(&v, 2), 4);
    }

    #[test]
    fn u_max() {
        let v = vec![1, 2, 3];
        assert_eq!(upper_bound(&v, 5), 3);
    }

    #[test]
    fn l_max() {
        let v = vec![1, 2, 3];
    }
}
