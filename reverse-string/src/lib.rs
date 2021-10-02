pub fn reverse(input: &str) -> String {
    input.chars().rev().fold(Vec::new(), |mut acc, character| {
        acc.push(character.to_string());
        acc
    }).join("")
}
