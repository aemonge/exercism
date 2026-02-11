use lingo::Lingo;

pub fn reverse(input: &str) -> String {
    let mut a = String::new();
    for c in input.chars().rev() {
        a.push(c);
    }
    a
}

// pub fn reverse(input: &str) -> String {
//     let mut a = String::new();
//     let textcat = Lingo::new();
//     let language = textcat.get_language(input);
//     println!("{input}");
//     dbg!(language);
//     println!("buuuuu");
//
//     for c in input.chars().rev() {
//         a.push(c);
//     }
//     a
// }
