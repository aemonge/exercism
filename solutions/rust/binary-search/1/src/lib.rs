pub fn find_(array: &[i32], key: i32, acc_ix: usize) -> Option<usize> {
    match array {
        [] => None,
        &[e] if e == key => Some(acc_ix),
        &[e] if e != key => None,
        &[_, e] if e == key => Some(acc_ix + 1),
        rest => {
            let ix = array.len() / 2;
            let (left, right) = rest.split_at(ix);
            match array[ix] {
                e if e > key => {
                    // dbg!("dead left");
                    find_(left, key, ix - 1)
                }
                e if e < key => {
                    // dbg!("dead right");
                    find_(right, key, acc_ix + ix)
                }
                _ => {
                    // dbg!("dies");
                    Some(ix + acc_ix)
                } // e == key
            }
        }
    }
}
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    find_(array, key, 0)
}
