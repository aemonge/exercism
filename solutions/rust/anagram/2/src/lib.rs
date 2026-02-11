use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

fn _was_anagram(word: &str, anagram: &str) -> bool {
    let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
    word_chars.sort();
    let mut anagram_chars: Vec<char> = anagram.to_lowercase().chars().collect();
    anagram_chars.sort();

    word_chars == anagram_chars
}

fn is_anagram(word: &str, anagram: &str) -> bool {
    let word_lowered = word.to_lowercase().clone();
    let mut word_chars: Vec<&str> = word_lowered.graphemes(true).collect();
    word_chars.sort();

    let anagram_lowered = anagram.to_lowercase().clone();
    let mut anagram_chars: Vec<&str> = anagram_lowered.graphemes(true).collect();
    anagram_chars.sort();

    dbg!(&word_chars, &anagram_chars);
    let is = word_chars == anagram_chars;
    dbg!(is);
    is
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut r: HashSet<&'a str> = HashSet::new();
    for anagram in possible_anagrams.iter() {
        println!("----------------------");
        dbg!(word, anagram);
        if is_anagram(word, anagram) {
            r.insert(anagram);
        };
    }
    r
}
