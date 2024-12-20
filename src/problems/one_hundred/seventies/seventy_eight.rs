pub fn run() {
    println!("{}", dp(1000000));
}

fn dp(d: u64) -> u32 {
    let mut dp = vec![];
    dp.push(vec![1u64]);
    for n in 1.. {
        let mut vec = vec![];
        vec.push(1);
        for m in 2..=n {
            let mut val = *vec.last().unwrap();
            let o = n - m;
            let row = &dp[o as usize];
            val += match row.get((m - 1) as usize) {
                Some(a) => *a,
                None => row.last().unwrap().clone(),
            };
            vec.push(val % d);
        }
        if vec.last().unwrap().clone() % d == 0 {
            return n;
        }
        dp.push(vec);
    }
    unreachable!()
}