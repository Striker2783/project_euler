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
shape_numbers!(Heptagonal, heptagonal);
#[inline(always)]
fn heptagonal(n: u32) -> Option<u32> {
    Some(n.checked_mul(n.checked_mul(5)? - 3)? / 2)
}
shape_numbers!(Octagonal, octagonal);
#[inline(always)]
fn octagonal(n: u32) -> Option<u32> {
    n.checked_mul(n.checked_mul(3)? - 2)
}
