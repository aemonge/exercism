use itertools::Itertools;
use std::cmp::max;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct MyError;

fn is_leading_zero(map: &HashMap<char, u8>, input: &str) -> bool {
    input
        .trim()
        .chars()
        .next()
        .and_then(|c| map.get(&c))
        .is_some_and(|&x| x == 0)
}

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
    let left_no_leading_zero: bool = left
        .split('+')
        .all(|word| !is_leading_zero(map, word));

    let right_sum: Result<usize, MyError> = right
        .split('+')
        .try_fold(0, |acc, word| Ok(acc + word_to_int(map, word)?));
    let right_no_leading_zero: bool = right
        .split('+')
        .all(|word| !is_leading_zero(map, word));

    Ok(left_no_leading_zero && right_no_leading_zero && left_sum? == right_sum?)
}

fn find_number_one(
    r: &mut HashMap<char, u8>,
    set: &mut HashSet<char>,
    left: &str,
    right: &str,
) -> bool {
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
        true
    } else {
        false
    }
}

fn find_number_x_by_adding_one(
    r: &mut HashMap<char, u8>,
    set: &mut HashSet<char>,
    left: &str,
    right: &str,
) -> Option<u8> {
    let stacked_left: Vec<&str> = left
        .split('+')
        .collect();
    let rows = stacked_left.len();
    let first_word = stacked_left.first();

    if let Some(word) = first_word {
        for (ix, e) in word
            .chars()
            .enumerate()
        {
            if r.get(&e) == Some(&1)
                && stacked_left
                    .iter()
                    .all(|word| {
                        word.chars()
                            .nth(ix)
                            .is_some_and(|x| r.get(&x) == Some(&1))
                    })
                && let Some(char_adds_by_only_one) = right
                    .chars()
                    .nth(ix)
            {
                let x: u8 = rows as u8;
                r.insert(char_adds_by_only_one, x);
                set.remove(&char_adds_by_only_one);
                return Some(x);
            }
        }
    }
    None
}

fn init_digits(letters: &HashSet<char>, left: &str, right: &str) -> Vec<u8> {
    let mut set: HashSet<char> = letters.clone();
    let mut r: HashMap<char, u8> = HashMap::new();
    let mut digits: Vec<u8> = (0u8..=9).collect();

    if find_number_one(&mut r, &mut set, left, right) {
        digits.remove(1);
        if let Some(x) = find_number_x_by_adding_one(&mut r, &mut set, left, right) {
            digits.remove(x as usize);
        }
    };
    digits
}

fn roll_letter_map(
    letters: &HashSet<char>,
    mut digits: Vec<u8>,
) -> Option<HashMap<char, u8>> {
    let set: HashSet<char> = letters.clone();
    let mut r: HashMap<char, u8> = HashMap::new();

    for e in set {
        let poped = digits.pop()?;
        r.insert(e, poped);
    }
    Some(r)
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left, right) = input.split_once("==")?;
    let (left, right) = (left.trim(), right.trim());
    let letters: HashSet<char> = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    let digits = init_digits(&letters, left, right);
    let digits_len = digits.len();
    let mut permutations = digits
        .into_iter()
        .permutations(digits_len);

    let mut candidate = roll_letter_map(&letters, permutations.next()?)?;
    while let Ok(false) = is_valid(&candidate, left, right) {
        candidate = roll_letter_map(&letters, permutations.next()?)?;
    }
    Some(candidate)
}
