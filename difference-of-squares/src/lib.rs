pub fn square_of_sum(n: u32) -> i32 {
    create_range(n).fold(0, |acc, num| acc + num).pow(2)
}

pub fn sum_of_squares(n: u32) -> i32 {
    create_range(n).fold(0, |acc, num| acc + num.pow(2))
}

pub fn difference(n: u32) -> i32 {
    square_of_sum(n) - sum_of_squares(n)
}

fn create_range(n: u32) -> std::ops::Range<i32> {
    0..(n as i32 + 1)
}
