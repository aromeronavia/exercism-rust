#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let vector = inputs.iter().fold(Vec::new(), |mut acc, input| {
        match input {
            CalculatorInput::Add => {
                if acc.len() < 2 {
                    return Vec::new()
                } else {
                    let operator1 = acc.pop().unwrap();
                    let operator2 = acc.pop().unwrap();

                    acc.push(operator1 + operator2);
                }
            },
            CalculatorInput::Subtract => {
                let operator1 = acc.pop().unwrap();
                let operator2 = acc.pop().unwrap();

                acc.push(operator2 - operator1);
            },
            CalculatorInput::Multiply => {
                let operator1 = acc.pop().unwrap();
                let operator2 = acc.pop().unwrap();

                acc.push(operator1 * operator2);
            },
            CalculatorInput::Divide => {
                let operator1 = acc.pop().unwrap();
                let operator2 = acc.pop().unwrap();

                acc.push(operator2 / operator1);
            },
            CalculatorInput::Value(number) => acc.push(*number)
        }
        acc
    });

    if vector.len() == 1 {
        return Some(vector[0]);
    } else if vector.len() > 1 {
        return None
    }

    return None
}
