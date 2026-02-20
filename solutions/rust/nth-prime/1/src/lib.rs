pub fn nth(i: u32) -> u32 {
    let n = 2 + i;
    let mut list: Vec<u32> = (2..(n.pow(2))).collect();
    for x in 2..n {
        list.retain(|n| n == &x || n % x != 0);
    }
    dbg!(&list);
    list[i as usize]
}
