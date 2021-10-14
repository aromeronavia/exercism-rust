pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let characters = string
        .chars()
        .filter(|x| {
            !x.is_whitespace() &&
            *x == '{' || *x == '}' ||
            *x == '(' || *x == ')' ||
            *x == '[' || *x == ']'
        });

    for character in characters {
        if character == '{' || character == '(' || character == '[' {
            stack.push(character);
        } else {
            match stack.pop() {
                Some(value) => {
                    if (value == '{' && character != '}') ||
                        (value == '[' && character != ']') ||
                        (value == '(' && character != ')')
                    {
                        stack.push(value);
                        stack.push(character);
                    }
                },
                None => stack.push(character)
            }
        }

    }

    stack.len() == 0
}
