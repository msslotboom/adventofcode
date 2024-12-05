use std::fs;

fn find_mas(start_index: i32, start_column: i32, content_array: &Vec<Vec<char>>) -> bool {
    let mut lowering_mas: Vec<&char> = vec![];
    lowering_mas.push(&content_array[(start_index - 1) as usize][(start_column - 1) as usize]);
    lowering_mas.push(&content_array[start_index as usize][start_column as usize]);
    lowering_mas.push(&content_array[(start_index + 1) as usize][(start_column + 1) as usize]);

    let mut rising_mas: Vec<&char> = vec![];
    rising_mas.push(&content_array[(start_index + 1) as usize][(start_column - 1) as usize]);
    rising_mas.push(&content_array[start_index as usize][start_column as usize]);
    rising_mas.push(&content_array[(start_index - 1) as usize][(start_column + 1) as usize]);

    let lowering_str = lowering_mas.iter().cloned().collect::<String>();
    let lowering_str_rev = lowering_mas.iter().cloned().rev().collect::<String>();
    if lowering_str != "MAS" && lowering_str_rev != "MAS" {
        return false;
    } else {
        let rising_str = rising_mas.iter().cloned().collect::<String>();
        let rising_str_rev = rising_mas.iter().cloned().rev().collect::<String>();
        if rising_str != "MAS" && rising_str_rev != "MAS" {
            return false;
        } else {
            return true;
        }
    }
}

fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let content_vec: Vec<&str> = file_contents.lines().collect();
    let content_array: Vec<Vec<char>> = content_vec
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let column_amount = content_array[0].len();
    let row_amount = content_array.len();
    let mut counter = 0;
    println!("{} {}", row_amount, column_amount);
    for row_index in 1..row_amount - 2 {
        for column_index in 1..column_amount - 1 {
            if content_array[row_index][column_index] == 'A' {
                if find_mas(row_index as i32, column_index as i32, &content_array) {
                    // println!("{} {}", row_index, column_index);
                    counter += 1;
                }
            }
        }
    }
    println!("{}", counter);
}
