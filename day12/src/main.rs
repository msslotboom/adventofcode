use std::fs;

fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let content_vec: Vec<Vec<char>> = file_contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut processed_chars: Vec<Vec<bool>> = vec![vec![false; content_vec[0].len()]; content_vec.len()];

    let mut total_score = 0;
    for row_index in 0..content_vec.len(){
        for col_index in 0..content_vec[0].len(){
            if processed_chars[row_index][col_index] == false {
                let score_of_group = part1(&content_vec, row_index, col_index, content_vec[row_index][col_index], &mut processed_chars);
                total_score += score_of_group.0 * score_of_group.1
            }
        }
    }
    println!("{}", total_score);

}

fn part1(char_table: &Vec<Vec<char>>, row_index: usize, col_index: usize, current_char: char, processed_chars: &mut Vec<Vec<bool>>) -> (i32, i32) {
    if processed_chars[row_index][col_index] {return (0, 0)}
    else if char_table[row_index][col_index] != current_char { return (0, 0) }
    else {
        processed_chars[row_index][col_index] = true;
        let neigbour_coords: Vec<(i32, i32)> = vec![(row_index as i32 - 1, col_index as i32), (row_index as i32 + 1 as i32, col_index as i32), (row_index as i32, col_index as i32 + 1), (row_index as i32, col_index as i32 - 1)];
        let mut return_value = (1, 0);
        let mut same_nbrs = 0;
        for coord in neigbour_coords{
            if coord.0 >= 0 && coord.1 >= 0 && coord.0 < char_table.len() as i32 && coord.1 < char_table[0].len() as i32{
                if char_table[coord.0 as usize][coord.1 as usize] == current_char {
                    same_nbrs += 1;
                    let rn_val = part1(char_table, coord.0 as usize, coord.1 as usize, current_char, processed_chars);
                    return_value = (return_value.0 + rn_val.0, return_value.1 + rn_val.1);
                    
                }
            }
        }
        let score = 4 - same_nbrs;
        return_value.1 += score;
        return return_value
    }
}
