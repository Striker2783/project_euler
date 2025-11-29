use common::is_prime;

pub fn run() {
    println!("{}", solve());
}

fn helper(v: [bool; 9], res: &mut Vec<u32>, curr_idx: usize, f: &mut impl FnMut(&[u32])) {
    if curr_idx != 0
        && curr_idx >= res.len()
        && (!is_prime(res[curr_idx - 1]) || (curr_idx > 1 && res[curr_idx - 2] > res[curr_idx - 1]))
    {
        return;
    }
    if v.iter().all(|b| !*b) {
        if curr_idx < res.len() {
            helper(v, res, curr_idx + 1, f);
        } else {
            f(res);
        }
        return;
    }
    let mut v_copy = v;
    for (i, _) in v.iter().cloned().enumerate().filter(|b| b.1) {
        let n = i as u32 + 1;
        if curr_idx >= res.len() {
            res.push(n);
            v_copy[i] = false;
            helper(v_copy, res, curr_idx, f);
            v_copy[i] = true;
            res.pop();
        } else {
            res[curr_idx] *= 10;
            res[curr_idx] += n;
            v_copy[i] = false;
            helper(v_copy, res, curr_idx, f);
            v_copy[i] = true;
            res[curr_idx] /= 10;
        }
    }
    if curr_idx < res.len() {
        helper(v, res, curr_idx + 1, f);
    }
}

fn solve() -> u32 {
    let mut count = 0;
    helper([true; 9], &mut vec![], 0, &mut |_| {
        count += 1;
    });
    count
}
