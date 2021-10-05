pub fn is_armstrong_number(num: u32) -> bool {
    let splitted_number = num
        .to_string()
        .chars()
        .map(|x| x as i32 - 0x30)
        .collect::<Vec<i32>>();

    let number = splitted_number.iter().fold(0i32, |acc, number| {
        acc + (number.pow(splitted_number.len() as u32))
    });
    num as i32 == number
}
