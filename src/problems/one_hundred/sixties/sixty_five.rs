use std::{
    cmp::PartialEq,
    convert::From,
    mem::swap,
    ops::{self, Add, AddAssign, Deref, DivAssign, Mul, MulAssign, Rem, RemAssign},
};

use num::BigUint;

pub fn run() {
    println!("{}", solve(100));
}
fn solve(n: usize) -> u32 {
    let mut iter: Vec<_> = ECF::default().take(n).map(|a| BigUint::from(a)).collect();
    let mut a = continued_fraction(iter.into_iter().rev()).unwrap().0;
    let mut sum = 0;
    while a > BigUint::ZERO {
        let b = (a.clone() % BigUint::from(10u32)).to_u64_digits();
        sum += match b.last() {
            Some(a) => a.clone(),
            None => 0,
        };
        a /= BigUint::from(10u32);
    }
    sum as u32
}
fn continued_fraction<T: Iterator<Item = U>, U>(mut iter: T) -> Option<Fraction<U>>
where
    U: From<u8> + Mul<Output = U>,
    U: Clone,
    U: DivAssign,
    U: Ord,
    U: RemAssign,
    U: AddAssign,
{
    let mut frac = Fraction::new(iter.next()?, U::from(1));
    for n in iter {
        frac.inverse();
        frac += n;
        frac.simplify();
    }
    Some(frac)
}
#[derive(Debug, Clone, Copy, Default)]
struct ECF {
    k: u32,
    n: u32,
}
impl Iterator for ECF {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let a = match self.n {
            0 => 2,
            1 => 1,
            _ => {
                if self.n % 3 == 2 {
                    self.k += 1;
                    2 * self.k
                } else {
                    1
                }
            }
        };
        self.n += 1;
        Some(a)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Fraction<T>(T, T);
impl<T> Fraction<T> {
    fn new(num: T, den: T) -> Self {
        Self(num, den)
    }
    pub fn inverse(&mut self) {
        swap(&mut self.0, &mut self.1);
    }
}
impl<T> Fraction<T>
where
    T: Ord + Clone + RemAssign + From<u8> + DivAssign,
{
    pub fn simplify(&mut self) -> &mut Self {
        let gcf = gcf(self.0.clone(), self.1.clone());
        self.0 /= gcf.clone();
        self.1 /= gcf;
        self
    }
}
impl<T> Add<T> for Fraction<T>
where
    T: Mul<Output = T> + Add<Output = T> + Clone,
{
    type Output = Fraction<T>;

    fn add(self, rhs: T) -> Self::Output {
        Self::new(self.0.add(self.1.clone().mul(rhs)), self.1)
    }
}
impl<T> AddAssign<T> for Fraction<T>
where
    T: Mul<Output = T> + AddAssign<T> + Clone,
{
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs * self.1.clone();
    }
}
impl<T> Add for Fraction<T>
where
    T: Mul<Output = T> + Add<Output = T> + Clone,
{
    type Output = Fraction<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.0 * rhs.1.clone() + self.1.clone() * rhs.0,
            rhs.1 * self.1,
        )
    }
}
impl<T> AddAssign for Fraction<T>
where
    T: Mul<Output = T> + MulAssign + AddAssign<T> + Clone,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += self.1.clone() * rhs.0;
        self.1 *= rhs.1;
    }
}

fn gcf<T>(mut a: T, mut b: T) -> T
where
    T: PartialEq + From<u8> + Clone + RemAssign,
{
    while b != T::from(0) {
        swap(&mut a, &mut b);
        b %= a.clone();
    }
    a
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::sixties::sixty_five::{
        continued_fraction, gcf, solve, Fraction,
    };

    use super::ECF;

    #[test]
    fn test_gcf() {
        assert_eq!(gcf(10, 2), 2);
        assert_eq!(gcf(10000, 7), 1);
        assert_eq!(gcf(200, 80), 40);
    }
    #[test]
    fn test_add() {
        assert_eq!(Fraction::new(100, 10) + 2, Fraction::new(120, 10));
        assert_eq!(
            Fraction::new(15, 5) + Fraction::new(30, 2),
            Fraction::new(180, 10)
        );
        assert_eq!(*Fraction::new(100, 20).simplify(), Fraction::new(5, 1));
    }
    #[test]
    fn test_continued_fraction() {
        assert_eq!(
            continued_fraction(vec![2].into_iter()),
            Some(Fraction::new(2, 1))
        );
        assert_eq!(
            continued_fraction(vec![2, 1, 2, 1, 1].into_iter().rev()),
            Some(Fraction::new(19, 7))
        );
    }
    #[test]
    fn test_ecf() {
        let mut iter = ECF::default();
        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 4);
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(1), 2);
        assert_eq!(solve(3), 8);
        assert_eq!(solve(5), 10);
        assert_eq!(solve(7), 7);
        assert_eq!(solve(10), 17);
    }
}
