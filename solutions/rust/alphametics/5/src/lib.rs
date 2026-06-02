use rand::seq::SliceRandom;
use std::cmp::max;
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

fn is_valid(map: &HashMap<char, u8>, left: &str, right: &str) -> Result<bool, MyError> {
    let left_sum: Result<usize, MyError> = left
        .split('+')
        .try_fold(0, |acc, word| Ok(acc + word_to_int(map, word)?));

    let right_sum: Result<usize, MyError> = right
        .split('+')
        .try_fold(0, |acc, word| Ok(acc + word_to_int(map, word)?));

    Ok(left_sum? == right_sum?)
}

fn find_number_one(
    r: &mut HashMap<char, u8>,
    set: &mut HashSet<char>,
    left: &str,
    right: &str,
) {
    let biggest_left = left
        .split('+')
        .fold(0, |acc, word| {
            max(
                acc,
                word.chars()
                    .count(),
            )
        });

    if right
        .chars()
        .count()
        > biggest_left
    {
        let the_one_char = right
            .chars()
            .next()
            .unwrap_or(' ');

        r.insert(the_one_char, 1);
        set.remove(&the_one_char);
    }
}

fn roll_letter_map(
    letters: &HashSet<char>,
    left: &str,
    right: &str,
) -> HashMap<char, u8> {
    let set: &mut HashSet<char> = letters.clone();
    let r: &mut HashMap<char, u8> = HashMap::new();
    let digits: &mut Vec<u8> = (0u8..=9).collect();
    let mut rng = rand::rng();

    find_number_one(r, &set, left, right);
    digits.shuffle(&mut rng);
    for e in *set {
        r.insert(
            e,
            digits
                .pop()
                .unwrap_or(0),
        );
    }
    *r
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left, right) = input.split_once("==")?;
    let (left, right) = (left.trim(), right.trim());
    let letters: HashSet<char> = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    let mut max = 1000;
    let mut candidate = roll_letter_map(&letters, left, right);
    while let Ok(false) = is_valid(&candidate, left, right)
        && max > 0
    {
        candidate = roll_letter_map(&letters, left, right);
        max -= 1;
    }
    if max == 0 { None } else { Some(candidate) }
}
