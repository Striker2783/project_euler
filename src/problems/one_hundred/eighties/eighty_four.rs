use std::{collections::HashMap, u32};

use rand::Rng;
const CC: [u32; 2] = [0, 10];
const CC_TOTAL: u32 = 16;
const C: [u32; 10] = [0, 10, 11, 24, 39, 5, 40, 40, 41, 42];
const C_TOTAL: u32 = 16;
pub fn run() {
    // println!("{}", monte_carlo(6));
    println!("{}", monte_carlo(4));
}
fn solve(rolls: u32) -> String {
    let results = simulate(rolls);
    let mut v = vec![0f64; 40];
    for ((k, _), c) in simulate(6) {
        v[k as usize] += c;
    }
    let mut v: Vec<_> = v.iter().enumerate().collect();
    v.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    println!("{v:?}");
    let mut str = String::new();
    for i in 0usize..3 {
        str.push_str(format!("{:02}", v[i].0).as_str());
    }
    return str;
}
fn get_shuffled_decks(arr: &[u32], size: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut new = vec![u32::MAX; size];
    for i in 0..arr.len() {
        new[i] = arr[i];
    }
    for i in 0..size {
        new.swap(i, rng.gen_range(0usize..size));
    }
    new
}
fn simulate_once(rolls: u32, v: &mut [u32]) {
    let c = get_shuffled_decks(&C, C_TOTAL as usize);
    let cc = get_shuffled_decks(&CC, CC_TOTAL as usize);
    let mut c_curr = 0;
    let mut cc_curr = 0;
    let mut place = 0;
    let mut doubles = 0;
    let mut rng = rand::thread_rng();
    let mut count = 0;
    for _ in 0..1_000 {
        let roll1 = rng.gen_range(1..=(rolls as usize));
        let roll2 = rng.gen_range(1..=(rolls as usize));
        if roll1 == roll2 && doubles >= 2 {
            place = 10;
            doubles = 0;
            count += 1;
        } else if roll1 == roll2 {
            doubles += 1;
            place = (place + roll1 + roll2) % 40;
        } else {
            doubles = 0;
            place = (place + roll1 + roll2) % 40;
        }
        match place {
            7 | 22 | 36 => {
                place = match c[c_curr] {
                    40 => match place {
                        7 => 15,
                        22 => 25,
                        36 => 5,
                        _ => unreachable!(),
                    },
                    41 => match place {
                        7 => 12,
                        22 => 28,
                        36 => 12,
                        _ => unreachable!(),
                    },
                    42 => place - 3,
                    u32::MAX => place,
                    _ => c[c_curr] as usize,
                };
                c_curr = (c_curr + 1) % (C_TOTAL as usize);
            }
            2 | 17 | 33 => {
                place = match cc[cc_curr] {
                    u32::MAX => place,
                    _ => cc[cc_curr] as usize,
                };
                cc_curr = (cc_curr + 1) % (CC_TOTAL as usize)
            }
            30 => {
                place = 10;
            }
            _ => (),
        }
        v[place] += 1;
    }
}
fn monte_carlo(rolls: u32) -> String {
    let mut visited = vec![0; 40];
    for _ in 0..1_000 {
        simulate_once(rolls, &mut visited);
    }
    let mut v: Vec<_> = visited.iter().enumerate().collect();
    v.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    v.iter().take(3).map(|n| format!("{:02}", n.0)).collect()
}
fn simulate(rolls: u32) -> HashMap<(u32, i32), f64> {
    let mut map = HashMap::new();
    map.insert((0, 0), 1f64);
    for _ in 0..1_000 {
        let mut new_map = HashMap::new();
        for ((mut place, mut doubles), chance) in map {
            for r1 in 1..=rolls {
                for r2 in 1..=rolls {
                    let place = (place + r1 + r2) % 40;
                    let doubles = if r1 == r2 { doubles + 1 } else { 0 };
                    let add = chance / (rolls * rolls) as f64;
                    if doubles == 3 {
                        new_map
                            .entry((10, 0))
                            .and_modify(|v| *v += add)
                            .or_insert(add);
                        continue;
                    }
                    match place {
                        7 | 22 | 36 => {
                            for c in C {
                                let mut place = match c {
                                    40 => match place {
                                        7 => 15,
                                        22 => 25,
                                        36 => 5,
                                        _ => unreachable!(),
                                    },
                                    41 => match place {
                                        7 => 12,
                                        22 => 28,
                                        36 => 12,
                                        _ => unreachable!(),
                                    },
                                    42 => place - 3,
                                    _ => c,
                                };
                                new_map
                                    .entry((place, doubles))
                                    .and_modify(|v| *v += add / C_TOTAL as f64)
                                    .or_insert(add / C_TOTAL as f64);
                            }
                            new_map
                                .entry((place, doubles))
                                .and_modify(|v| {
                                    *v += add * (C_TOTAL - C.len() as u32) as f64 / C_TOTAL as f64
                                })
                                .or_insert(
                                    add * (C_TOTAL - C.len() as u32) as f64 / C_TOTAL as f64,
                                );
                        }
                        2 | 17 | 33 => {
                            for c in CC {
                                new_map
                                    .entry((c, doubles))
                                    .and_modify(|v| *v += add / CC_TOTAL as f64)
                                    .or_insert(add / CC_TOTAL as f64);
                            }
                            new_map
                                .entry((place, doubles))
                                .and_modify(|v| {
                                    *v +=
                                        add * (CC_TOTAL - CC.len() as u32) as f64 / CC_TOTAL as f64
                                })
                                .or_insert(
                                    add * (CC_TOTAL - CC.len() as u32) as f64 / CC_TOTAL as f64,
                                );
                        }
                        30 => {
                            new_map
                                .entry((10, doubles))
                                .and_modify(|v| *v += add)
                                .or_insert(add);
                        }
                        _ => {
                            new_map
                                .entry((place, doubles))
                                .and_modify(|v| *v += add)
                                .or_insert(add);
                        }
                    };
                }
            }
        }
        map = new_map;
    }
    map
}
#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::eighties::eighty_four::monte_carlo;

    use super::simulate_once;

    #[test]
    fn test_simulate_once() {
        assert_eq!(monte_carlo(6), "102400".to_string());
    }
}