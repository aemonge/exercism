pub fn find_<T>(array: &[T], key: &T, acc_ix: usize) -> Option<usize>
where
    T: PartialOrd + Copy,
{
    match array {
        [] => None,
        &[e] if e == *key => Some(acc_ix),
        &[e] if e != *key => None,
        &[_, e] if e == *key => Some(acc_ix + 1),
        rest => {
            let ix = array.len() / 2;
            let (left, right) = rest.split_at(ix);
            match array[ix] {
                e if e > *key => find_(left, key, acc_ix),
                e if e < *key => find_(right, key, acc_ix + ix),
                _ => Some(ix + acc_ix), // e == key
            }
        }
    }
}

pub fn find<T, K>(array: K, key: T) -> Option<usize>
where
    T: PartialOrd + Copy,
    K: AsRef<[T]>,
{
    find_(array.as_ref(), &key, 0)
}
