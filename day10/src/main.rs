use std::{collections::HashSet, fs};

fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let matrix: Vec<Vec<u32>> = file_contents
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|digit| digit)
                .collect()
        })
        .collect();
    
    let mut trailheads = vec![];

    for (row_index, row) in matrix.iter().enumerate(){
        for (col_index, num) in row.iter().enumerate(){
            if *num == 0 { trailheads.push((row_index, col_index)) }
        }
    }
    let mut score1 = 0;
    let mut score2 = 0;
    for trailhead in trailheads{
        score1 += part1(trailhead, matrix.len()-1, matrix[0].len()-1, &matrix, 0).len();
        let trail_score = part2(trailhead, matrix.len()-1, matrix[0].len()-1, &matrix, 0);
        score2 += trail_score;
    }
    println!("{}", score1);
    println!("{}", score2);
}

fn part1(coord: (usize, usize), max_row: usize, max_col: usize, matrix: &Vec<Vec<u32>>, current_nr: u32) -> HashSet<(usize, usize)> {
    let mut nines = HashSet::new();
    let steps: [(i32, i32); 4] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    ];
    for step in steps{
        let new_row = coord.0 as i32 + step.0;
        let new_col = coord.1 as i32 + step.1;
        if new_row >= 0 && new_col >= 0 && new_row <= max_row as i32 && new_col <= max_col as i32 {
            let new_nr = matrix[new_row as usize][new_col as usize];
            if current_nr + 1 == new_nr {
                if new_nr == 9 { 
                    nines.insert((new_row as usize, new_col as usize));
                }
                for nine in part1((new_row as usize, new_col as usize), max_row, max_col, matrix, new_nr){
                    nines.insert((nine.0, nine.1));
                }
                }
            }
            else {
                continue;
            }
    }
    return nines;
}

fn part2(coord: (usize, usize), max_row: usize, max_col: usize, matrix: &Vec<Vec<u32>>, current_nr: u32) -> i32 {
    let mut score = 0;
    let steps: [(i32, i32); 4] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    ];
    for step in steps{
        let new_row = coord.0 as i32 + step.0;
        let new_col = coord.1 as i32 + step.1;
        if new_row >= 0 && new_col >= 0 && new_row <= max_row as i32 && new_col <= max_col as i32 {
            let new_nr = matrix[new_row as usize][new_col as usize];
            if current_nr + 1 == new_nr {
                if new_nr == 9 { 
                    score += 1
                }
                score += part2((new_row as usize, new_col as usize), max_row, max_col, matrix, new_nr)
                }
            }
            else {
                continue;
            }
    }
    return score;
}