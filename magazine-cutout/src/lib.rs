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
    let mut magazine_map: HashMap<&str, i8> = HashMap::new();
    let magazine_words = magazine.iter().fold(magazine_map, build_map);

    let mut note_map: HashMap<&str, i8> = HashMap::new();
    let note_words = note.iter().fold(note_map, build_map);

    note_words.iter().fold(true, |acc, (word, value)| {
        if magazine_words.get(word) < Some(value) {
            acc && false
        } else {
            acc && true
        }
    })
}
