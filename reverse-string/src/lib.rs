pub fn reverse(input: &str) -> String {
    let mut inverted_string: Vec<String> = Vec::new();
    for i in input.chars().rev() {
        inverted_string.push(i.to_string())
    }

    inverted_string.join("")
}
