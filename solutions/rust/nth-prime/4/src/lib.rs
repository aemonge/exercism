fn multiples(k: u32, x: u32) -> Vec<u32> {
    (1..=x)
        .map(|i| k * i)
        .collect()
}

fn remove_multiples(list: Vec<u32>, i: u32) -> Vec<u32> {
    let mut r: Vec<u32> = vec![list[0]];

    let multiples = multiples(list[0], i);
    let i_m = 0;

    for e in list {
        if e == multiples[i_m] {
            i_m += 1;
        } else {
            r.push(e);
        }
    }
}
fn reduce(list: Vec<u32>, i: u32, max: u32) -> Vec<u32> {
    let mut r: Vec<u32> = vec![list[0]];
    let mut n;
    let ln = list.len() as u32;

    for e in list {
        dbg!(e);
        for x in 2..ln {
            n = i * x;
            if e != n {
                dbg!(e);
                r.push(e);
            }
        }
    }
    // for x in list {
    //     if (x == i) || (x % i != 0) {
    //         r.push(x);
    //     }
    // }
    if i == max { r } else { reduce(r, i + 1, max) }
}

pub fn nth(i: u32) -> u32 {
    let n = 2 + i;
    let list = reduce((2..(n.pow(2))).collect(), 2, n);
    dbg!(&list);
    list[i as usize]
}
