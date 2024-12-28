use std::collections::HashSet;

pub fn run() {
    let mut count = 0;
    every(|a| {
        every(|b| {
            if possible(a, b) {
                count += 1;
            }
        })
    });
    println!("{}", count / 2);
}
type Dice = HashSet<u32>;
fn possible(a: &Dice, b: &Dice) -> bool {
    let helper = |n: u32| -> bool {
        (a.contains(&(n % 10)) && b.contains(&(n / 10)))
            || (b.contains(&(n % 10)) && a.contains(&(n / 10)))
    };
    helper(1)
        && helper(4)
        && (helper(6) || helper(9))
        && (helper(16) || helper(19))
        && helper(25)
        && (helper(36) || helper(39))
        && (helper(46) || helper(49))
        && (helper(94) || helper(64))
        && helper(81)
}

fn every<T>(mut f: T)
where
    T: FnMut(&Dice),
{
    fn recurse<T>(mut f: &mut T, dice: &mut Dice, start: u32)
    where
        T: FnMut(&Dice),
    {
        if dice.len() >= 6 {
            f(dice);
            return;
        } else if 10 - (start as usize) < 5 - dice.len() {
            return;
        }
        for n in start..10 {
            dice.insert(n as u32);
            recurse(f, dice, n + 1);
            dice.remove(&(n as u32));
        }
    }
    let mut set = HashSet::new();
    recurse(&mut f, &mut set, 0);
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::problems::one_hundred::nineties::ninety::possible;

    use super::every;

    #[test]
    fn test_possible() {
        let a = [0, 5, 6, 7, 8, 9].into_iter().collect();
        let b = [1, 2, 3, 4, 8, 9].into_iter().collect();
        assert!(possible(&a, &b));
    }
    #[test]
    fn test_every() {
        let mut count = 0;
        let mut no_duplicates = HashSet::new();
        every(|d| {
            count += 1;
            let mut v: Vec<_> = d.into_iter().cloned().collect();
            v.sort();
            assert!(no_duplicates.insert(v));
        });
        assert_eq!(count, 210);
    }
}
