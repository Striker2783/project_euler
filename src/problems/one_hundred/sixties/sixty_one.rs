use std::collections::{HashMap, HashSet};

use common::{
    number_series::Squares,
    shape_numbers::{Heptagonal, Hexagonal, Octagonal, Pentagonal, Triangular},
};
use num::pow::Pow;

pub fn run() {
    let mut solver = Solver::new(4, 6);
    println!("{}", solver.solve());
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Triangle,
    Square,
    Pentagon,
    Hexagon,
    Heptagon,
    Octagon,
}
#[derive(Debug, Default)]
struct Solver {
    map: HashMap<u8, Vec<(Shape, u32)>>,
    digits: u32,
    shapes: usize,
}
impl Solver {
    pub fn new(digits: u32, shapes: usize) -> Self {
        Self {
            digits,
            shapes,
            ..Default::default()
        }
    }
    fn solve(mut self) -> u32 {
        self.add_shapes();
        let mut alg = RecursionAlgorithm::new();
        alg.solve(self)
    }
    fn add_shapes(&mut self) {
        self.add_nums(Triangular::default(), Shape::Triangle);
        if self.shapes < 2 {
            return;
        }
        self.add_nums(Squares::default(), Shape::Square);
        if self.shapes < 3 {
            return;
        }
        self.add_nums(Pentagonal::default(), Shape::Pentagon);
        if self.shapes < 4 {
            return;
        }
        self.add_nums(Hexagonal::default(), Shape::Hexagon);
        if self.shapes < 5 {
            return;
        }
        self.add_nums(Heptagonal::default(), Shape::Heptagon);
        if self.shapes < 6 {
            return;
        }
        self.add_nums(Octagonal::default(), Shape::Octagon);
    }
    fn add_nums(&mut self, s: impl Iterator<Item = u32>, e: Shape) {
        for n in s {
            if n < 10u32.pow(self.digits - 1) {
                continue;
            } else if n >= 10u32.pow(self.digits) {
                break;
            }
            self.map
                .entry((n / 100).try_into().unwrap())
                .and_modify(|v| v.push((e, n)))
                .or_insert(vec![(e, n)]);
        }
    }
}

struct RecursionAlgorithm {
    nums: Vec<u32>,
    results: HashSet<Vec<u32>>,
    set: HashSet<Shape>,
    first: Shape,
}
impl RecursionAlgorithm {
    pub fn new() -> Self {
        Self {
            nums: vec![],
            results: HashSet::new(),
            set: HashSet::default(),
            first: Shape::Heptagon,
        }
    }
    unsafe fn get_end(&self) -> Vec<u32> {
        self.results
            .iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .clone()
    }
    pub fn solve(&mut self, d: Solver) -> u32 {
        for (&h, v) in &d.map {
            for (s, n) in v.clone() {
                self.set.insert(s);
                self.nums.push(n);
                self.first = s;
                unsafe { self.recurse(&d) };
                self.set.remove(&s);
                self.nums.pop();
            }
        }
        let v = unsafe { self.get_end() };
        v.iter().sum()
    }
    unsafe fn recurse(&mut self, d: &Solver) {
        if self.nums.len() > d.shapes {
            return;
        }
        let v = d.map.get(
            &(self.nums.last().unwrap().clone() % 100)
                .try_into()
                .unwrap(),
        );
        let v = match v {
            None => return,
            Some(a) => a,
        };
        for (s, n) in v.clone() {
            if self.nums.len() > 1 && s == self.first && n == self.nums[0] {
                let mut clone = self.nums.clone();
                clone.sort();
                self.results.insert(clone);
                continue;
            }
            if self.set.contains(&s) {
                continue;
            }
            self.set.insert(s);
            self.nums.push(n);

            unsafe { self.recurse(&d) };
            self.set.remove(&s);
            self.nums.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use common::shape_numbers::Triangular;

    use crate::problems::one_hundred::sixties::sixty_one::Shape;

    use super::Solver;

    #[test]
    fn test_solve() {
        let mut solver = Solver::new(4, 3);
        assert_eq!(solver.solve(), 8128 + 2882 + 8281);
    }

    #[test]
    fn test_add_nums() {
        let mut solver = Solver::new(1, 6);
        solver.add_nums(Triangular::default(), super::Shape::Triangle);
        let expected: HashMap<_, _> = vec![(
            0,
            vec![
                (Shape::Triangle, 1),
                (Shape::Triangle, 3),
                (Shape::Triangle, 6),
            ],
        )]
        .into_iter()
        .collect();
        assert_eq!(expected, solver.map);
        solver.add_shapes();
        let expected: HashMap<_, _> = vec![(
            0,
            vec![
                (Shape::Triangle, 1),
                (Shape::Triangle, 3),
                (Shape::Triangle, 6),
                (Shape::Triangle, 1),
                (Shape::Triangle, 3),
                (Shape::Triangle, 6),
                (Shape::Square, 1),
                (Shape::Square, 4),
                (Shape::Square, 9),
                (Shape::Pentagon, 1),
                (Shape::Pentagon, 5),
                (Shape::Hexagon, 1),
                (Shape::Hexagon, 6),
                (Shape::Heptagon, 1),
                (Shape::Heptagon, 7),
                (Shape::Octagon, 1),
                (Shape::Octagon, 8),
            ],
        )]
        .into_iter()
        .collect();
        assert_eq!(expected, solver.map);
    }
}
