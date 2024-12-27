use std::{collections::HashMap, fs};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Position{
    row: usize,
    col: usize,
    direction: char
}

fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let grid_as_str = file_contents.split("&").nth(0).unwrap();
    let mut grid: Vec<Vec<char>> = vec![];
    let mut pos= Position{row:0, col:0, direction:' '};
    for (row_index, row) in grid_as_str.lines().enumerate(){
        let mut row_of_grid = vec![];
        for item in row.char_indices(){
            if item.1 == 'S' {
                pos = Position{row:row_index, col:item.0, direction:' '};
            }
            row_of_grid.push(item.1)
        }
        grid.push(row_of_grid);
    }

    let mut scores = vec![];
    if pos.row as i32 - 1 >= 0{
        if grid[pos.row - 1][pos.col] == '.'{
            let new_pos = Position{row: pos.row+1, col: pos.col, direction: '^'};
            let mut position_map: HashMap<Position, i64> = HashMap::new();
            position_map.insert(new_pos.clone(), 1001);
            scores.push(path_finding(&grid, new_pos, &'^', 1001, &mut position_map));
        }
    }
    if pos.row + 1 < grid.len(){
        if grid[pos.row + 1][pos.col] == '.'{
            let new_pos = Position{row: pos.row-1, col: pos.col, direction: 'v'};
            let mut position_map: HashMap<Position, i64> = HashMap::new();
            position_map.insert(new_pos.clone(), 1001);
            scores.push(path_finding(&grid, new_pos, &'v', 1001, &mut position_map));
        }
    }
    if pos.col as i32 - 1 >= 0{
        if grid[pos.row][pos.col-1] == '.'{
            let new_pos = Position{row: pos.row, col: pos.col-1, direction: '<'};
            let mut position_map: HashMap<Position, i64> = HashMap::new();
            position_map.insert(new_pos.clone(), 2001);
            scores.push(path_finding(&grid, new_pos, &'<', 2001, &mut position_map));
        }
    }
    if pos.col + 1 < grid[0].len(){
        if grid[pos.row][pos.col+1] == '.'{
            let new_pos = Position{row: pos.row, col: pos.col+1, direction: '>'};
            let mut position_map: HashMap<Position, i64> = HashMap::new();
            position_map.insert(new_pos.clone(), 1);
            scores.push(path_finding(&grid, new_pos, &'>', 1, &mut position_map));
        }
    }
    println!("{:?}", scores);
}

fn path_finding(grid: &Vec<Vec<char>>, pos: Position, current_direction: &char, score: i64, path_map: &mut HashMap<Position, i64>) -> Option<i64> {
    if path_map.contains_key(&pos){
        if path_map[&pos] < score { return None }
    }
    let directions = vec!['v', '>', '^', '<'];
    let mut score_vector = vec![];
    for new_direction in directions{
        let new_pos = update_pos(&pos, new_direction);
        if grid[new_pos.row][new_pos.col] == 'E' {
            return Some(score + score_of_turning(current_direction, &new_direction) + 1);
        }
        if grid[new_pos.row][new_pos.col] == '.'{
            let score_of_turn = score_of_turning(current_direction, &new_direction);
            let score_of_new_path = score + score_of_turn + 1;
            if path_map.contains_key(&new_pos){
                if path_map[&new_pos] < score { continue; }
            };
            path_map.insert(new_pos.clone(), score_of_new_path);
            let score_of_path = path_finding(grid, new_pos, &new_direction, score_of_new_path, path_map);
            match score_of_path {
                Some(x) => score_vector.push(x),
                None => continue,
            }
        }
    }
    if score_vector.len() == 0 { return None }
    score_vector.sort();
    println!("{:?}", score_vector);
    return Some(score_vector[0])
}

fn score_of_turning(dir_1: &char, dir_2: &char) -> i64 {
    let directions = vec!['v', '>', '^', '<'];
    if dir_1 == dir_2 { return 0 }
    let index_1 = directions.clone().into_iter().position(|c| c == *dir_1).unwrap() as i32;
    let index_2 = directions.into_iter().position(|c| c == *dir_2).unwrap() as i32;
    if (index_1 - index_2).abs() == 1 || (index_1 - index_2).abs() == 3{
        return 1000
    }
    else if (index_1 - index_2).abs() == 2 {
        return 2000
    }
    else{
        println!("Something went terribly wrong");
        return 1
    }
}

fn update_pos(pos:&Position, direction:char) -> Position {
    match direction{
        'v' => {
            return Position{row: pos.row+1, col:pos.col, direction:pos.direction}
        }
        '^' => {
            return Position{row: pos.row-1, col:pos.col, direction:pos.direction}
        }
        '>' => {
            return Position{row: pos.row, col:pos.col+1, direction:pos.direction}
        }
        '<' => {
            return Position{row: pos.row, col:pos.col-1, direction:pos.direction}
        }
        _ => {
            println!("Something wrong: {}", direction);
            return Position{row:0, col:0, direction:pos.direction};
        }
    }
}