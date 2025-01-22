use std::{f64::EPSILON, fs};

const FILE: &str = "Files/102.txt";
pub fn run() {
    let s = fs::read_to_string(FILE).unwrap();
    println!("{}", solve(&s));
}
fn solve(s: &str) -> u32 {
    let mut count = 0;
    for line in s.lines() {
        if Triangle::from(line).contains_origin() {
            count += 1;
        }
    }
    count
}
type Point = (i32, i32);
struct Triangle([Point; 3]);
impl Triangle {
    fn area(a: &Point, b: &Point, c: &Point) -> f64 {
        let ab_x = b.0 - a.0;
        let ab_y = b.1 - a.1;

        let ac_x = c.0 - a.0;
        let ac_y = c.1 - a.1;

        let cross = ab_x * ac_y - ab_y * ac_x;

        0.5 * f64::abs(cross as f64)
    }
    fn contains_origin(&self) -> bool {
        let a1 = Self::area(&(0, 0), &self.0[0], &self.0[1]);
        let a2 = Self::area(&(0, 0), &self.0[0], &self.0[2]);
        let a3 = Self::area(&(0, 0), &self.0[1], &self.0[2]);
        let a = Self::area(&self.0[0], &self.0[1], &self.0[2]);
        (a1 + a2 + a3 - a).abs() < EPSILON
    }
}
impl From<&str> for Triangle {
    fn from(value: &str) -> Self {
        let mut split = value.split(',');
        let mut arr = [(0, 0); 3];
        for i in 0..3 {
            arr[i] = (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        }
        Self(arr)
    }
}
#[cfg(test)]
mod tests {
    use crate::tens::two::Triangle;

    #[test]
    fn test_contains_origin() {
        assert!(Triangle::from("-340,495,-153,-910,835,-947").contains_origin());
        assert!(!Triangle::from("-175,41,-421,-714,574,-645").contains_origin());
    }
    #[test]
    fn test_triangle() {
        let triangle = Triangle::from("0,0,0,3,4,0");
        let epsilon = 0.00001;
        assert!(
            (Triangle::area(&triangle.0[0], &triangle.0[1], &triangle.0[2]) - 6.0).abs() < epsilon
        );
    }
}
