use std::{collections::HashSet, ops::Add};

pub fn run() {
    let set: HashSet<_> = PentagonalNumbers::default().collect();
    let min = PentagonalNumbers::default()
        .filter_map(|a| {
            PentagonalNumbers::default()
                .filter_map(|b| {
                    if set.contains(&a.add(b)) && set.contains(&b.abs_diff(a)) {
                        Some(b.abs_diff(a))
                    } else {
                        None
                    }
                })
                .min()
        })
        .min()
        .unwrap();
    println!("{min}");
}

#[derive(Debug, Default)]
struct PentagonalNumbers {
    n: u32,
}
impl Iterator for PentagonalNumbers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;
        let checked = self.n.checked_mul(3 * self.n - 1);
        if let Some(m) = checked {
            Some(m / 2)
        } else {
            None
        }
    }
}
