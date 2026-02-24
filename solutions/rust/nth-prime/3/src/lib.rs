// fn reduce(mut list: &Vec<u32>, i: u32) {
//     list.retain(|n| n == &i || n % i != 0);
// }

fn reduce(list: Vec<u32>, i: u32, max: u32) -> Vec<u32> {
    // if max == 0 {
    //     return vec![2];
    // }
    // if max == 1 {
    //     return vec![2, 3];
    // }
    let mut r: Vec<u32> = vec![];
    for x in list {
        if (x == i) || (x % i != 0) {
            r.push(x);
        }
    }
    if i == max { r } else { reduce(r, i + 1, max) }
}

pub fn nth(i: u32) -> u32 {
    let n = 2 + i;
    // let list: Vec<u32> = (2..(n.pow(2))).collect();
    let list = reduce((2..(n.pow(2))).collect(), 2, n);
    // for j in 0..i {
    //     let x = list[j as usize];
    //     dbg!(x);
    //     list.retain(|n| n == &x || n % x != 0);
    //     // reduce(&list, x + 1);
    // }
    // for x in 2..n {
    //     list.retain(|n| n == &x || n % x != 0);
    // }
    dbg!(&list);
    list[i as usize]
}
