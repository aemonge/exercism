use std::collections::{HashMap, HashSet};

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

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left, right) = input.split_once("==")?;
    let (left, right) = (left.trim(), right.trim());
    let letters: HashSet<char> = input
        .chars()
        .collect();
    dbg!(input);
    dbg!(letters, left, right);
    todo!("Solve the alphametic {input:?}")
}
