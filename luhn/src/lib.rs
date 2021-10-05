/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let number = code
        .chars()
        .filter(|x| !x.is_whitespace())
        .map(|x| x as i32 - 0x30)
        .collect::<Vec<i32>>();

    if number.len() <= 1 {
        return false;
    }

    if number.iter().any(|x| *x < 0 || *x > 9) {
        return false;
    }

    let sum = number.iter().rev().enumerate().fold(0, |acc, (index, x)| {
        if index % 2 == 1 {
            let mut product = x * 2;
            if product > 9 {
                product -= 9;
            }

            println!("X: {}, product: {}", x, product);
            acc + product
        } else {
            println!("X: {}", x);
            acc + x
        }
    });

    println!("Sum: {}", sum);

    sum % 10 == 0
}
