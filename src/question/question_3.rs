use std::collections::{HashSet, HashMap};

pub fn part_1(input: &String) -> String {
    input
        .lines()
        .fold(0, |acc, x| 
            acc + calculate_prio(get_duplicate(x)))
        .to_string()
}

pub fn part_2(input: &String) -> String {
    let lines: Vec<&str> = input
        .lines()
        .collect();

    lines.chunks(3)
        .fold(0, |acc, next| 
            acc + calculate_prio(get_duplicate_three(next)))
        .to_string()
        
}

fn get_duplicate_three(input: &[&str]) -> char {
    let uniques:Vec<HashSet<char>> = input
        .iter()
        .map(|&x| x.chars().map(|c| c).collect())
        .collect();
        
    let charas = uniques
        .iter()
        .flatten() 
        .fold(HashMap::new(), |mut acc:HashMap<char, usize>, next| {
            let number = acc.entry(*next).or_insert(0);
            *number += 1;
            return acc;
        });

    return *charas.iter().find(|x| x.1 == &3).expect("msg").0
}

fn get_duplicate(input: &str) -> char {
    let half: usize = input.len() / 2;
    let (first, second) = input.split_at(half);

    let unique: HashSet<char> = first
        .chars()
        .map(|c| c)
        .collect();

    for c in unique {
        if second.contains(c) {
            return c;
        }
    }

    panic!("{}", input)
}

fn calculate_prio(c: char) -> usize {
    if c.is_uppercase() {
        return c as usize - 'A' as usize + 27;
    } else {
        return c as usize - 'a' as usize + 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::question::question_3::calculate_prio;

    #[test]
    fn calculate_lower_case() {
        let result = calculate_prio('c');
        assert_eq!(result, 3);
    }

    #[test]
    fn calculate_upper_case() {
        let result = calculate_prio('L');
        assert_eq!(result, 38);
    }
}