use std::fmt::Debug;
use std::fmt::Write; // Memory buffer, NOT stdout

fn start_bottles_to_string(start_bottles: u32) -> String {
    match start_bottles {
        1 => String::from("One"),
        2 => String::from("Two"),
        3 => String::from("Three"),
        4 => String::from("Four"),
        5 => String::from("Five"),
        6 => String::from("Six"),
        7 => String::from("Seven"),
        8 => String::from("Eight"),
        9 => String::from("Nine"),
        10 => String::from("Ten"),
        _ => String::from("No"),
    }
}

fn plularize_bottle(start_bottles: u32) -> String {
    if start_bottles == 1 {
        String::from("bottle")
    } else {
        String::from("bottles")
    }
}

fn _recite(start_bottles: u32, take_down: u32) -> Result<String, std::fmt::Error> {
    let mut r = String::new();
    let range = ((start_bottles - take_down + 1)..(start_bottles + 1)).rev();
    for start_bottles in range {
        let number = start_bottles_to_string(start_bottles);
        let number_less = start_bottles_to_string(start_bottles - 1).to_lowercase();
        let n_bot = plularize_bottle(start_bottles);
        let nl_bot = plularize_bottle(start_bottles - 1);

        writeln!(r, "{number} green {n_bot} hanging on the wall,")?;
        writeln!(r, "{number} green {n_bot} hanging on the wall,")?;
        writeln!(r, "And if one green bottle should accidentally fall,")?;
        writeln!(
            r,
            "There'll be {number_less} green {nl_bot} hanging on the wall.",
        )?;
        writeln!(r, "")?;
    }
    Ok(String::from(r.trim()))
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    match _recite(start_bottles, take_down) {
        Ok(r) => r,
        _ => String::from("Super unknow error..."),
    }
}
