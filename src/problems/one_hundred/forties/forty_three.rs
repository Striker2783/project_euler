use crate::common::SMALL_PRIMES;

pub fn run() {
    let mut permutations = Permutations::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let mut count = 0;
    permutations.run(|arr| {
        if is_sub_divisible(arr) {
            count += arr_to_num(arr);
        };
    });
    println!("{count}"); // Should be 16695334890
}

fn arr_to_num(arr: &[u32]) -> u64 {
    let mut n = 0;
    for &x in arr {
        n *= 10;
        n += x as u64;
    }
    n
}

fn is_sub_divisible(arr: &[u32]) -> bool {
    let mut n = arr[1] * 10 + arr[2];
    for i in 0..(arr.len() - 3) {
        n %= 100;
        n *= 10;
        n += arr[i + 3];
        if n % SMALL_PRIMES[i] != 0 {
            return false;
        }
    }
    true
}
struct Permutations<T: Default + Copy, const N: usize> {
    permutation: [T; N],
    active: [bool; N],
    curr: [T; N],
}

impl<T: Default + Copy, const N: usize> Permutations<T, N> {
    fn helper<U: FnMut(&[T])>(&mut self, i: usize, f: &mut U) {
        if i == N {
            f(&self.curr);
            return;
        }
        for j in 0..N {
            if self.active[j] {
                continue;
            }
            self.curr[i] = self.permutation[j];
            self.active[j] = true;
            self.helper(i + 1, f);
            self.active[j] = false;
        }
    }
    fn run<U: FnMut(&[T])>(&mut self, mut f: U) {
        self.helper(0, &mut f);
    }
    fn new(permutation: [T; N]) -> Self {
        Self {
            permutation,
            active: [false; N],
            curr: [T::default(); N],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{is_sub_divisible, Permutations};

    #[test]
    fn is_sub_divisible_test() {
        assert!(is_sub_divisible(&[1, 4, 0, 6, 3, 5, 7, 2, 8, 9]));
    }
    #[test]
    fn permutations() {
        let mut perms = Permutations::new([0, 1, 2, 3]);
        let mut count = 0;
        perms.run(|_| count += 1);

        assert_eq!(count, 4 * 3 * 2)
    }
}
