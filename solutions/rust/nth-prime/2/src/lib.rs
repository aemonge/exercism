// fn reduce(mut list: &Vec<u32>, i: u32) {
//     list.retain(|n| n == &i || n % i != 0);
// }

pub fn nth(i: u32) -> u32 {
    let n = 2 + i;
    let mut list: Vec<u32> = (2..(n.pow(2))).collect();
    for j in 0..i {
        let x = list[j as usize];
        dbg!(x);
        list.retain(|n| n == &x || n % x != 0);
        // reduce(&list, x + 1);
    }
    // for x in 2..n {
    //     list.retain(|n| n == &x || n % x != 0);
    // }
    dbg!(&list);
    list[i as usize]
}
