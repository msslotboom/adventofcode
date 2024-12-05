use std::{char, fs};

fn count_xmas(row: &str) -> i32 {
    let fwd = row.matches("XMAS").count();
    let bwd = row
        .chars()
        .rev()
        .collect::<String>()
        .matches("XMAS")
        .count();
    return (fwd + bwd).try_into().unwrap();
}
fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let content_vec: Vec<&str> = file_contents.lines().collect();
    let mut counter = 0;
    let row_count = content_vec.len();

    // lowering diagonals
    let mut lowering_diagonals: Vec<String> = vec![];
    for start_index in (0..row_count - 4).rev() {
        let mut column_index = 0;
        let mut diag: Vec<char> = vec![];
        for i in start_index..row_count - 1 {
            diag.push(content_vec[i].chars().nth(column_index).unwrap().clone());
            column_index += 1;
        }
        let diag_as_str = diag.iter().cloned().collect::<String>();
        lowering_diagonals.push(diag_as_str)
    }
    let column_length = content_vec[0].len();
    for start_column_index in 1..column_length - 3 {
        let mut row_index = 0;
        let mut diag: Vec<char> = vec![];
        for column_index in start_column_index..column_length {
            diag.push(
                content_vec[row_index]
                    .chars()
                    .nth(column_index)
                    .unwrap()
                    .clone(),
            );
            row_index += 1;
        }
        let diag_as_str = diag.iter().cloned().collect::<String>();
        lowering_diagonals.push(diag_as_str);
    }
    // rising diagonals
    let mut rising_diagonals: Vec<String> = vec![];
    for start_row in 4..row_count {
        let mut column_index = 0;
        let mut diag: Vec<char> = vec![];
        for row_index in (0..start_row).rev() {
            diag.push(
                content_vec[row_index]
                    .chars()
                    .nth(column_index)
                    .unwrap()
                    .clone(),
            );
            column_index += 1;
        }
        let diag_as_str = diag.iter().cloned().collect::<String>();
        rising_diagonals.push(diag_as_str);
    }
    for start_column in 1..column_length - 3 {
        let mut row_index = row_count - 2;
        let mut diag: Vec<char> = vec![];
        for column_index in start_column..column_length {
            diag.push(
                content_vec[row_index]
                    .chars()
                    .nth(column_index)
                    .unwrap()
                    .clone(),
            );
            row_index -= 1;
        }
        let diag_as_str = diag.iter().cloned().collect::<String>();
        rising_diagonals.push(diag_as_str);
    }
    // columns
    let mut columns: Vec<String> = vec![];
    for j in 0..column_length {
        let mut col: Vec<char> = vec![];
        for i in 0..row_count - 1 {
            col.push(content_vec[i].chars().nth(j).unwrap().clone());
        }
        let col_as_str = col.iter().cloned().collect::<String>();
        columns.push(col_as_str);
    }
    for row in content_vec {
        counter += count_xmas(row);
    }
    for row in lowering_diagonals {
        counter += count_xmas(&row);
    }
    for row in rising_diagonals {
        counter += count_xmas(&row);
    }
    for col in columns {
        counter += count_xmas(&col);
    }
    println!("{}", counter);
}
