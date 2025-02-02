use std::collections::HashMap;
use std::io::prelude::BufRead;
use std::io::stdin;
use text_word_stats::{display, sort};

fn main() {
    let input = stdin();
    let handel = input.lock();
    let mut words = HashMap::new();
    for line in handel.lines() {
        let line = line.unwrap();
        for word in line.split_whitespace() {
            let word = word.to_lowercase();
            let word = word.trim_matches(|c: char| !c.is_alphabetic());
            let count = words.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
    }
    let sorted = sort(words);
    display(sorted);

}
