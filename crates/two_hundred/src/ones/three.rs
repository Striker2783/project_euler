pub fn run() {
    let a = find(7);
    let res: String = a.into_iter().map(|n| n.to_string()).collect();
    println!("{res}");
}

fn find(n: usize) -> Vec<u32> {
    let a = generate_suboptimal(n);
    let mut min = u32::MAX;
    let mut res = Vec::new();
    vary(&a, 4, |v| {
        let sum: u32 = v.iter().sum();
        if sum < min && is_special_sum_set(v) {
            min = sum;
            res = v.to_vec();
        }
    });
    res
}

fn vary(v: &[u32], diff: u32, mut f: impl FnMut(&[u32])) {
    let mut result = Vec::new();
    fn vary_helper(v: &[u32], diff: u32, result: &mut Vec<u32>, f: &mut impl FnMut(&[u32])) {
        if v.is_empty() {
            f(result);
            return;
        }
        result.push(v[0]);
        if result.is_sorted() {
            vary_helper(&v[1..], diff, result, f);
        }
        result.pop();
        for n in 1..=diff {
            result.push(v[0] - n);
            if result.is_sorted() {
                vary_helper(&v[1..], diff, result, f);
            }
            result.pop();
            result.push(v[0] + n);
            if result.is_sorted() {
                vary_helper(&v[1..], diff, result, f);
            }
            result.pop();
        }
    }
    vary_helper(v, diff, &mut result, &mut f);
}

/// Only checks for greater than mainly
fn condition2(v: &[u32]) -> bool {
    if v.len() <= 2 {
        return true;
    }
    let mut prefix_sums = vec![0; v.len()];
    prefix_sums[0] = v[0];
    for i in 1..v.len() {
        prefix_sums[i] = prefix_sums[i - 1] + v[i];
    }
    for i in 1..v.len().div_ceil(2) {
        let prefix = prefix_sums[i];
        let suffix = prefix_sums[v.len() - 1] - prefix_sums[v.len() - i - 1];
        if suffix >= prefix {
            return false;
        }
    }
    true
}

fn get_sum(v: &[u32], mut idx: u32) -> u32 {
    let mut sum = 0;
    for &n in v {
        if idx & 1 == 1 {
            sum += n;
        }
        idx >>= 1;
    }
    sum
}

fn condition1(v: &[u32]) -> bool {
    for subset1 in 1u32..(1 << v.len()) {
        for subset2 in (subset1 + 1)..(1 << v.len()) {
            if subset1 == subset2 || subset1 & subset2 != 0 {
                continue;
            }
            let (ones1, ones2) = (subset1.count_ones(), subset2.count_ones());
            if ones1 != ones2 {
                continue;
            }
            let (sum1, sum2) = (get_sum(v, subset1), get_sum(v, subset2));
            if sum1 == sum2 {
                return false;
            }
        }
    }
    true
}

pub fn is_special_sum_set(v: &[u32]) -> bool {
    condition1(v) && condition2(v)
}

fn generate_suboptimal(n: usize) -> Vec<u32> {
    let mut prev = vec![1];
    for i in 1..n {
        let mut next = vec![0; i + 1];
        let mid_idx = if i % 2 == 1 { (i - 1) / 2 } else { i / 2 };
        let mid = prev[mid_idx];
        next[0] = mid;
        for i in 0..i {
            next[i + 1] = mid + prev[i];
        }
        prev = next;
    }
    prev
}

#[cfg(test)]
mod tests {
    use crate::ones::three::{
        condition1, condition2, find, generate_suboptimal, is_special_sum_set,
    };

    #[test]
    fn test_generate_suboptimal() {
        assert_eq!(generate_suboptimal(1), [1]);
        assert_eq!(generate_suboptimal(2), [1, 2]);
        assert_eq!(generate_suboptimal(3), [2, 3, 4]);
        assert_eq!(generate_suboptimal(4), [3, 5, 6, 7]);
        assert_eq!(generate_suboptimal(5), [6, 9, 11, 12, 13]);
        assert_eq!(generate_suboptimal(6), [11, 17, 20, 22, 23, 24]);
    }
    #[test]
    fn test_is_special_sum_set() {
        for i in 1..=6 {
            assert!(is_special_sum_set(&generate_suboptimal(i)));
        }
    }
    #[test]
    fn test_condition2() {
        assert!(condition2(&[1, 1, 1]));
        assert!(!condition2(&[2, 4, 8]));
        assert!(!condition2(&[2, 4, 5, 8]));
        assert!(condition2(&[6, 9, 11, 12, 13]));
        assert!(condition2(&[11, 17, 20, 22, 23, 24]));
    }
    #[test]
    fn test_condition1() {
        assert!(condition1(&[11, 17, 20, 22, 23, 24]));
    }
    #[test]
    fn test_find() {
        assert_eq!(find(6), [11, 18, 19, 20, 22, 25]);
    }
}
