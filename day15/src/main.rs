use std::fs;

#[derive(Debug, Copy, Clone)]
struct Position{
    row: usize,
    col: usize
}
fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let grid_as_str = file_contents.split("&").nth(0).unwrap();
    let mut grid: Vec<Vec<char>> = vec![];
    let mut pos= Position{row:0, col:0};
    for (row_index, row) in grid_as_str.lines().enumerate(){
        let mut row_of_grid = vec![];
        for item in row.char_indices(){
            if item.1 == '@' {
                pos = Position{row:row_index, col:item.0};
            }
            row_of_grid.push(item.1)
        }
        grid.push(row_of_grid);
    }
    let moves = file_contents.split("&").nth(1).unwrap();

    for item in moves.char_indices(){
        if item.1 != '\n' {
            move_grid(item.1, &mut grid, pos);
            pos = find_rob_pos(&grid);
        }
    }
    let score = calculate_score(&grid);
    println!("{}", score);
}

fn find_rob_pos(grid: &Vec<Vec<char>>) -> Position{
    for (row_index, row) in grid.into_iter().enumerate(){
        for (col_index, c) in row.into_iter().enumerate(){
            if *c == '@'{
                return Position{row:row_index, col:col_index};
            }
        }
    }
    return Position{row:0, col:0};
}

fn update_pos(pos:Position, direction:char) -> Position {
    match direction{
        'v' => {
            return Position{row: pos.row+1, col:pos.col}
        }
        '^' => {
            return Position{row: pos.row-1, col:pos.col}
        }
        '>' => {
            return Position{row: pos.row, col:pos.col+1}
        }
        '<' => {
            return Position{row: pos.row, col:pos.col-1}
        }
        _ => {
            println!("Something wrong: {}", direction);
            return Position{row:0, col:0};
        }
    }
}

fn move_grid(direction: char, grid: &mut Vec<Vec<char>>, pos: Position){
    let mut positions_to_update: Vec<Position> = vec![];
    positions_to_update.push(pos);
    let mut coords: Position = pos;
    loop{
        let new_coords = update_pos(coords, direction);
        if grid[new_coords.row][new_coords.col] == '.'{
            // moving the pieces
            let mut new_pos = new_coords;
            for pos in positions_to_update.into_iter().rev() {
                grid[new_pos.row][new_pos.col] = grid[pos.row][pos.col];
                new_pos = pos;
            }
            grid[new_pos.row][new_pos.col] = '.';
            break;
        }
        else if grid[new_coords.row][new_coords.col] == '#'{
            break;
        }
        else{
            positions_to_update.push(new_coords);
            if grid[new_coords.row][new_coords.col] == '.' {}
            coords = new_coords;
        }
    }
    
}

fn calculate_score(grid: &Vec<Vec<char>>) -> i32 {
    let mut score = 0;
    for (row_index, row) in grid.into_iter().enumerate(){
        for (col_index, c) in row.into_iter().enumerate(){
            if *c == 'O'{
                score += ((100 * row_index) + col_index) as i32;
            }
        }
    }
    return score
}