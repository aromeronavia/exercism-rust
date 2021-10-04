pub fn get_minesweeper(matrix: &[&str]) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    for y in 0..matrix.len() {
        let mut string = String::new();
        for x in 0..matrix[y].len() {
            let current = matrix[y].chars().nth(x).unwrap();
            if current == '*' {
                string.push(current);
                continue;
            }

            let mut count = 0;
            let x_int = x as i8;
            let y_int = x as i8;

            if current != '*' {
                for xs in x_int-1..x_int+2 {
                    for ys in y_int-1..y_int+2 {
                        count += {
                            if xs < 0 || ys < 0 {
                                count
                            } else {
                                if ys as usize >= matrix.len() {
                                    0
                                }
                                else {
                                    match matrix[y_int as usize].chars().nth(x_int as usize) {
                                        Some(ch) => if ch == '*' { 1 } else { 0 },
                                        None => 0
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if count > 0 {
                string.push(count.to_string().chars().nth(0).unwrap());
            }
        }

        res.push(string);
    }

    res
}

pub fn annotate(minefield: & [&str]) -> Vec<String> {
    if minefield.len() > 0 {
        let result = get_minesweeper(minefield);
        return result;
    }
    Vec::new()
}
