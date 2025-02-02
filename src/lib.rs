use std::collections::HashMap;

pub fn sort(words: HashMap<String, i32>) -> Vec<(String, i32)> {
    let mut words: Vec<(String, i32)> = words.into_iter().collect();
    words.sort_by(|a, b| b.1.cmp(&a.1));
    words
}

pub fn display(input: Vec<(String, i32)>)    {
    for (word, count) in input {
        println!("{}: {}", word, count);
    }
}