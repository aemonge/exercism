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
                e if e > key => find_(left, key, acc_ix),
                e if e < key => find_(right, key, acc_ix + ix),
                _ => Some(ix + acc_ix), // e == key
            }
        }
    }
}
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    dbg!(&array, key);
    find_(array, key, 0)
}
