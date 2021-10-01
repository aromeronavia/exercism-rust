// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn build_map(mut acc: HashMap<&'static str, i8>, word: &&'static str) -> HashMap<&'static str, i8>{
    if !acc.contains_key(word) {
        acc.insert(word, 1);
    } else {
        acc.insert(word, acc[word] + 1);
    }
    acc
}

pub fn can_construct_note(magazine: &[&'static str], note: &[&'static str]) -> bool {
    let magazine_words = magazine.iter().fold(HashMap::new(), build_map);
    let note_words = note.iter().fold(HashMap::new(), build_map);

    note_words.iter().fold(true, |acc, (word, value)| {
        if magazine_words.get(word) < Some(value) {
            acc && false
        } else {
            acc && true
        }
    })
}
