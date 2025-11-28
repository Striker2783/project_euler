pub fn nine() {
    let triplets = pythagoreon_triples(&500);
    let max = triplets.filter(|x| x.0 + x.1 + x.2 == 1000).last();
    if let Some(a) = max {
        println!("{}", a.0 * a.1 * a.2);
    }
}
pub fn pythagoreon_triples(upper: &u64) -> impl Iterator<Item = (u64, u64, u64)> + '_ {
    (1..*upper).flat_map(|x| {
        (1..*upper).filter_map(move |y| {
            let x_squared = x * x;
            let y_squared = y * y;
            let c_squared = x_squared + y_squared;
            let c = (c_squared as f64).sqrt() as u64;
            if c * c == c_squared {
                Some((x, y, c))
            } else {
                None
            }
        })
    })
}
