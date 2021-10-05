pub fn verse(n: u32) -> String {
    if n == 0 {
        String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    } else {
        if n == 1 {
            return String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
        } else {
            let plural_first = if n > 1 { "s" } else { "" };
            let plural_second = if n - 1 > 1 { "s" } else { "" };

            let phrase = format!("{} bottle{} of beer on the wall, {} bottle{} of beer.\nTake one down and pass it around, {} bottle{} of beer on the wall.\n", n, plural_first, n, plural_first, n-1, plural_second);

            String::from(phrase)
        }
    }
}

pub fn get_phrase(i: i32, is_last: bool) -> String {
    if i > 1 {
        let plural_second = if i - 1 > 1 { "s" } else { "" };

        format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle{} of beer on the wall.\n{}", i, i, i-1, plural_second, if is_last {""} else {"\n"})
    } else if i == 1 {
        let plural_first = if i > 1 { "s" } else { "" };

        let phrase = format!("{} bottle{} of beer on the wall, {} bottle{} of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\n", i, plural_first, i, plural_first);

        String::from(phrase)
    } else {
        String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    }
}

pub fn sing(start: u32, end: u32) -> String {
    println!("{}, {}", start, end);
    let mut result: String = String::new();
    let mut i = start as i32;
    while i >= end as i32 {
        result.push_str(get_phrase(i, i == end as i32).as_str());
        println!("result {}", result);

        i-=1;
    }
    result
}
