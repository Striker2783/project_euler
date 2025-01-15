use common::shape_numbers::{Hexagonal, Pentagonal, Triangular};

pub fn run() {
    let mut solver = Solver::default();
    solver.next();
    solver.next();
    println!("{}", solver.next().unwrap());
}

#[derive(Debug, Default)]
struct Solver {
    tri: Triangular,
    pent: Pentagonal,
    hexa: Hexagonal,
}

impl Iterator for Solver {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut tri = self.tri.next()?;
        let mut pent = self.pent.next()?;
        let mut hexa = self.hexa.next()?;

        while tri != pent || pent != hexa {
            if tri < pent {
                tri = self.tri.next()?;
            } else if pent < hexa {
                pent = self.pent.next()?;
            } else {
                hexa = self.hexa.next()?;
            }
        }

        Some(tri)
    }
}

macro_rules! shape_numbers {
    ($name: ident, $func:ident) => {
        #[derive(Debug, Default)]
        pub struct $name {
            n: u32,
        }
        impl Iterator for $name {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                self.n += 1;
                $func(self.n)
            }
        }
    };
}
#[cfg(test)]
mod tests {
    use super::{Hexagonal, Pentagonal, Solver, Triangular};

    #[test]
    fn test_solver() {
        let mut solver = Solver::default();
        for expected in [1, 40755, 1533776805] {
            assert_eq!(solver.next().unwrap(), expected)
        }
    }
}
