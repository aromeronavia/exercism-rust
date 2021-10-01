// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map: HashMap<&str, i8> = HashMap::new();
    let magazine_words = magazine.iter().fold(magazine_map, |mut acc, word| {
        if !acc.contains_key(word) {
            acc.insert(word, 1);
        } else {
            acc.insert(word, acc[word] + 1);
        }
        acc
    });

    let mut note_map: HashMap<&str, i8> = HashMap::new();
    let note_words = note.iter().fold(note_map, |mut acc, word| {
        if !acc.contains_key(word) {
            acc.insert(word, 1);
        } else {
            acc.insert(word, acc[word] + 1);
        }

        acc
    });

    note_words.iter().fold(true, |acc, (word, value)| {
        // println!("{}-{}-{:?}-{:?}", word, value, magazine_words.get(word), Some(value));
        if magazine_words.get(word) < Some(value) {
            return acc && false;
        }

        return acc && true;
    })
}