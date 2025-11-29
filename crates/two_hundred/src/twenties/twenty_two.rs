pub fn run() {
    println!("{}", solve(200));
}

fn binary(mut e: u32) -> u32 {
    let original_e = e;
    let mut a = 0;
    while e > 0 {
        e /= 2;
        a += 1;
    }
    a + original_e.count_ones() - 2
}

fn solve(n: u32) -> u32 {
    (1..=n).map(mults).sum::<u32>()
}

fn mults_helper(target: u32, curr_v: &mut Vec<u32>, mut min_chain: usize) -> usize {
    assert!(!curr_v.is_empty());
    let curr = *curr_v.last().unwrap();
    if (curr > target) || curr_v.len() > min_chain {
        return min_chain;
    } else if curr == target {
        return curr_v.len() - 1;
    }

    for i in 0..curr_v.len() {
        let value = curr_v[i];
        curr_v.push(curr + value);
        let res = mults_helper(target, curr_v, min_chain);
        curr_v.pop();
        min_chain = min_chain.min(res);
    }
    min_chain
}

fn mults(e: u32) -> u32 {
    mults_helper(e, &mut vec![1], 
        binary(e) as usize
    ) as u32
}

#[cfg(test)]
mod tests {
    use crate::twenties::twenty_two::{binary, mults};

    #[test]
    fn test_mults() {
        assert_eq!(binary(15), 6);
        assert_eq!(binary(16), 4);

        assert_eq!(mults(1), 0);
        assert_eq!(mults(2), 1);
        assert_eq!(mults(3), 2);
        assert_eq!(mults(4), 2);
        assert_eq!(mults(5), 3);
        assert_eq!(mults(6), 3);
        assert_eq!(mults(7), 4);
        assert_eq!(mults(8), 3);
        assert_eq!(mults(9), 4);
        assert_eq!(mults(10), 4);
        assert_eq!(mults(15), 5);
    }
}
