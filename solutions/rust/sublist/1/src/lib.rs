use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn identical(first_list: &[i32], second_list: &[i32]) -> bool {
    let mut are_equal = true;

    for (i, val) in first_list.iter().enumerate() {
        are_equal &= val == &second_list[i];
    }

    are_equal
}

fn contained(bigger: &[i32], smaller: &[i32]) -> bool {
    let mut contained = false;
    let smaller_len = smaller.len();
    dbg!(smaller.len(), bigger.len(), bigger.len() - smaller.len());
    let a = bigger.len();
    let c = a - smaller_len;
    dbg!(&bigger[..c]);

    for (i, val) in bigger[..bigger.len() - smaller.len()].iter().enumerate() {
        if val == &smaller[0] {
            dbg!(contained);
            contained |= identical(&bigger[i..], smaller)
        }
    }

    dbg!(contained);
    contained
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Less => {
            let bigger = second_list;
            let smaller = first_list;

            match contained(bigger, smaller) {
                true => Comparison::Sublist,
                false => Comparison::Unequal,
            }
        }
        Ordering::Equal => match identical(first_list, second_list) {
            true => Comparison::Equal,
            false => Comparison::Unequal,
        },

        Ordering::Greater => {
            let bigger = first_list;
            let smaller = second_list;
            dbg!("bigger, {}", bigger.len());
            dbg!("smaller, {}", smaller.len());

            match dbg!(contained(bigger, smaller)) {
                true => Comparison::Superlist,
                false => Comparison::Unequal,
            }
        }
    }
}
