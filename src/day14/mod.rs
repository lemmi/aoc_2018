use super::*;
use std::iter::FromIterator;

fn digits(a: usize) -> Vec<u8> {
    let mut ret = Vec::new();
    let mut a = a;
    while a > 0 {
        let t = a;
        a /= 10;
        ret.push((t - a * 10) as u8);
    }
    ret.reverse();
    ret
}

fn digits_of_sum(a: u8, b: u8) -> Vec<u8> {
    let sum = a + b;
    if sum >= 10 {
        vec![1, sum - 10]
    } else {
        vec![sum]
    }
}

fn next_recipes(c1: usize, c2: usize, recipes: &[u8]) -> (usize, usize, Vec<u8>) {
    let r1 = recipes[c1];
    let r2 = recipes[c2];
    let new_recipes = digits_of_sum(r1, r2);
    let n = recipes.len() + new_recipes.len();
    (
        (c1 + r1 as usize + 1) % n,
        (c2 + r2 as usize + 1) % n,
        new_recipes,
    )
}

fn generate(n: usize) -> Vec<u8> {
    let mut c1 = 0;
    let mut c2 = 1;
    let mut recipes = vec![3, 7];

    while recipes.len() < n + 10 {
        let (n1, n2, new_recipes) = next_recipes(c1, c2, &recipes);
        recipes.extend(new_recipes);
        c1 = n1;
        c2 = n2;
    }

    recipes[n..n + 10].to_owned()
}

fn search(input: &[u8]) -> usize {
    let il = input.len();
    let mut c1 = 0;
    let mut c2 = 1;
    let mut recipes = vec![3, 7];

    loop {
        let (n1, n2, new_recipes) = next_recipes(c1, c2, &recipes);
        for nr in new_recipes.iter() {
            recipes.push(*nr);
            let rl = recipes.len();
            if rl > il && &recipes[rl - il..] == input {
                return rl - il;
            }
        }
        c1 = n1;
        c2 = n2;
    }
}

pub fn star1(input: usize) -> StarResult {
    println!(
        "Scores: {:?}",
        String::from_iter(generate(input).iter().map(|c| format!("{}", c)))
    );
    Ok(())
}
pub fn star2(input: usize) -> StarResult {
    let v = digits(input);
    println!("{} first appears after {} recipes.", input, search(&v));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples1() {
        assert_eq!(generate(9), digits(5158916779));
        assert_eq!(generate(5), vec![0, 1, 2, 4, 5, 1, 5, 8, 9, 1]);
        assert_eq!(generate(18), digits(9251071085));
        assert_eq!(generate(2018), digits(5941429882));
    }

    #[test]
    fn examples2() {
        assert_eq!(search(&vec![5, 1, 5, 8, 9]), 9);
        assert_eq!(search(&vec![0, 1, 2, 4, 5]), 5);
        assert_eq!(search(&vec![9, 2, 5, 1, 0]), 18);
        assert_eq!(search(&vec![5, 9, 4, 1, 4]), 2018);
    }
}
