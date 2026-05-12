fn to_bool_vec(n: u32, mut acc: Vec<bool>) -> Vec<bool> {
    if n > 1 {
        acc.push(!n.is_multiple_of(2));

        to_bool_vec(n / 2, acc)
    } else {
        acc.push(!n.is_multiple_of(2));

        acc
    }
}

pub fn egg_count(display_value: u32) -> usize {
    let v = to_bool_vec(display_value, vec![]);
    v.iter()
        .rev()
        .fold(
            0,
            |accumulator, &e| {
                if e { accumulator + 1 } else { accumulator }
            },
        )
}
