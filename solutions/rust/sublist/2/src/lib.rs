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
    let bigger_len = bigger.len();
    let verify_up_to = bigger_len - smaller_len + 1;
    let mut i = 0;

    // By definition
    if smaller_len == 0 {
        return true;
    }

    while i <= verify_up_to && !contained {
        if bigger[i] == smaller[0] {
            contained |= identical(&bigger[i..(smaller_len + i)], smaller)
        }

        i += 1;
    }

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

            match contained(bigger, smaller) {
                true => Comparison::Superlist,
                false => Comparison::Unequal,
            }
        }
    }
}
