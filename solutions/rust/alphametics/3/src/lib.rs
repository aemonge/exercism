use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct MyError;

fn word_to_int(map: &HashMap<char, u8>, input: &str) -> Result<usize, MyError> {
    let digits: String = input
        .trim()
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

fn is_valid(map: &HashMap<char, u8>, left: &str, right: &str) -> Result<bool, MyError> {
    let left_sum: Result<usize, MyError> = left
        .split('+')
        .try_fold(0, |acc, word| Ok(acc + word_to_int(map, word)?));
    dbg!(&left_sum);

    let right_sum: Result<usize, MyError> = right
        .split('+')
        .try_fold(0, |acc, word| Ok(acc + word_to_int(map, word)?));
    dbg!(&right_sum);

    Ok(left_sum? == right_sum?)
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left, right) = input.split_once("==")?;
    let (left, right) = (left.trim(), right.trim());
    let letters: HashSet<char> = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    let mut max = 1000;
    let mut candidate = roll_letter_map(&letters);
    while let Ok(false) = is_valid(&candidate, left, right)
        && max > 0
    {
        candidate = roll_letter_map(&letters);
        max -= 1;
    }
    dbg!(max);
    if max == 0 { None } else { Some(candidate) }

    // dbg!(input);
    // dbg!(left, right);
    // dbg!(&candidate);
    // dbg!(word_to_int(&candidate, right));
    // dbg!(is_valid(&candidate, left, right));
    // todo!("Solve the alphametic {input:?}")
}
