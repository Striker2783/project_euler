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

shape_numbers!(Triangular, triangle);
#[inline(always)]
fn triangle(n: u32) -> Option<u32> {
    Some(n.checked_mul(n + 1)? / 2)
}
shape_numbers!(Pentagonal, pentagonal);
#[inline(always)]
fn pentagonal(n: u32) -> Option<u32> {
    Some(n.checked_mul(3 * n - 1)? / 2)
}
shape_numbers!(Hexagonal, hexagonal);
#[inline(always)]
fn hexagonal(n: u32) -> Option<u32> {
    n.checked_mul(2 * n - 1)
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

    macro_rules! test_shapes {
        ($name:ident, $shape: ident, $values: expr) => {
            #[test]
            fn $name() {
                let mut shape = $shape::default();
                for expected in $values {
                    assert_eq!(shape.next().unwrap(), expected);
                }
            }
        };
    }
    test_shapes!(test_triangle, Triangular, [1, 3, 6, 10, 15]);
    test_shapes!(test_pentagon, Pentagonal, [1, 5, 12, 22, 35]);
    test_shapes!(test_hexagon, Hexagonal, [1, 6, 15, 28, 45]);
}
