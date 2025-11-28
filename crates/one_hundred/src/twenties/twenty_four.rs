
struct Permutations {
    n: Vec<u32>,
}
impl Permutations {
    pub fn new(n: u32) -> Self {
        let mut v = vec![];
        for i in 0..n {
            v.push(i);
        }
        Self { n: v }
    }
    pub fn get(&self) -> &[u32] {
        &self.n
    }
    fn get_ceil(arr: &[u32]) -> usize {
        let target = arr[0];
        let mut min = u32::MAX;
        let mut index = 0;
        for (i, &n) in arr.iter().skip(1).enumerate() {
            if n > target && n < min {
                min = n;
                index = i + 1;
            }
        }
        index
    }
    pub fn next(&mut self) {
        let mut first_swap = self.n.len() - 1;
        while first_swap > 0 {
            if self.n[first_swap - 1] < self.n[first_swap] {
                break;
            }
            first_swap -= 1;
        }
        if first_swap == 0 {
            self.n.sort();
            return;
        }
        first_swap -= 1;
        let second_swap = Self::get_ceil(&self.n[first_swap..]) + first_swap;
        self.n.swap(first_swap, second_swap);
        self.n[(first_swap + 1)..].sort();
    }
}

pub fn run() {
    let mut p = Permutations::new(10);
    for _ in 1..1_000_000 {
        p.next();
    }
    println!("{:?}", p.get());
}

#[cfg(test)]
mod tests {
    use super::Permutations;

    #[test]
    fn next() {
        let mut p = Permutations::new(3);
        assert_eq!(p.get(), [0, 1, 2]);
        p.next();
        assert_eq!(p.get(), [0, 2, 1]);
        p.next();
        assert_eq!(p.get(), [1, 0, 2]);
        p.next();
        assert_eq!(p.get(), [1, 2, 0]);
        p.next();
        assert_eq!(p.get(), [2, 0, 1]);
        p.next();
        assert_eq!(p.get(), [2, 1, 0]);
        p.next();
        assert_eq!(p.get(), [0, 1, 2]);
    }
    #[test]
    fn problem() {
        let mut p = Permutations::new(10);
        for _ in 1..1_000_000 {
            p.next();
        }
        assert_eq!(p.get(), [2, 7, 8, 3, 9, 1, 5, 4, 6, 0])
    }
}
