use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut word_vector = word.chars().into_iter().map(|x| x.to_lowercase().next()).collect::<Vec<_>>();
    word_vector.sort();

    let result: HashSet<&'a str> = HashSet::new();
    possible_anagrams.into_iter().fold(result, |mut acc, string: &&'a str| {
        if word == *string {
            acc
        } else if
            word.chars().into_iter().map(|x| x.to_lowercase().next()).collect::<Vec<_>>() ==
            string.chars().into_iter().map(|x| x.to_lowercase().next()).collect::<Vec<_>>()
         {
            acc
        } else {
            let mut string_vec = string.chars().into_iter().map(|x| x.to_lowercase().next()).collect::<Vec<_>>();
            string_vec.sort();
            if word_vector ==  string_vec {
                acc.insert(string);
            }
            acc
        }
    })
}
