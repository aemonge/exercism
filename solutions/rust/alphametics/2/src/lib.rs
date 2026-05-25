use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct MyError;

fn word_to_int(map: HashMap<char, u8>, input: &str) -> Result<usize, MyError> {
    let digits: String = input
        .chars()
        .map(|c| {
            map.get(&c)
                .map(|d| char::from(b'0' + *d))
        })
        .collect::<Option<String>>()
        .ok_or(MyError)?;

    digits
        .parse()
        .map_err(|_| MyError)
}

fn roll_letter_map(set: &HashSet<char>) -> HashMap<char, u8> {
    let mut r: HashMap<char, u8> = HashMap::new();
    let mut digits: Vec<u8> = (0u8..=9).collect();
    let mut rng = rand::rng();

    digits.shuffle(&mut rng);
    for e in set {
        r.insert(
            *e,
            digits
                .pop()
                .unwrap_or(0),
        );
    }
    r
}

fn is_valid(map: HashMap<char, u8>, left: &str, right: &str) -> Result<bool, MyError> {
    let left_sum = left
        .split('+')
        .try_fold(0, |acc, word| Ok(acc + word_to_int(map, word)?));

    let right_sum = right
        .split('+')
        .try_fold(0, |acc, word| Ok(acc + word_to_int(map, word)?));

    Ok(left_sum == right_sum)
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left, right) = input.split_once("==")?;
    let (left, right) = (left.trim(), right.trim());
    let letters: HashSet<char> = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    let mut candidate = roll_letter_map(&letters);

    dbg!(input);
    dbg!(left, right);
    dbg!(&candidate);
    dbg!(word_to_int(candidate, right));
    todo!("Solve the alphametic {input:?}")
}
