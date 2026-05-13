#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

fn to_base_10(num_vec: &[u32], from_base: u32) -> u32 {
    let mut n: u32 = 0;

    for (ix, e) in (0..num_vec.len())
        .rev()
        .zip(num_vec.iter())
    {
        n += e * from_base.strict_pow(ix as u32);
    }

    n
}

fn from_base_10(n: u32, to_base: u32, acc: &mut Vec<u32>) {
    if n >= to_base {
        acc.push(n % to_base);
        from_base_10(n / to_base, to_base, acc);
    } else {
        acc.push(n % to_base);
        acc.reverse();
    }
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(
    number: &[u32],
    from_base: u32,
    to_base: u32,
) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    if number
        .iter()
        .any(|&x| x >= from_base)
    {
        return Err(Error::InvalidDigit(from_base));
    }

    let mut r = Vec::new();
    let as_base_10 = to_base_10(number, from_base);
    from_base_10(as_base_10, to_base, &mut r);
    Ok(r)
}
