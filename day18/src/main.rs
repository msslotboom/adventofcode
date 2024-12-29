use std::{collections::{HashMap, VecDeque}, fs};
#[derive(Clone, Eq, PartialEq, Debug, Hash)]

struct Position{
    row: usize,
    col: usize
}

const row_limit: usize = 70;
const col_limit: usize = 70;

fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();

    // Initialise board
    let mut board = vec![];
    for _ in 0..71 {
        let mut row_of_board = vec![];
        for _ in 0..71 {
            row_of_board.push('.');
        }
        board.push(row_of_board);
    }

    let mut counter = 0;
    for row in file_contents.lines(){
        counter += 1;
        let col_index = row.split(",").nth(0).unwrap().parse::<usize>().unwrap();
        let row_index= row.split(",").nth(1).unwrap().trim().parse::<usize>().unwrap();
        board[row_index as usize][col_index] = '#';

        if counter > 1024{
            let start_pos = Position{ row: 0, col: 0};
            let mut path_map: HashMap<Position, i64> = HashMap::new();
            let best_score = path_finding(&board, start_pos, 0, &mut path_map);
            if best_score == None{
                println!("{}, {}", col_index, row_index);
                break;
            }
        }
    }
}

fn path_finding(grid: &Vec<Vec<char>>, pos: Position, score:i64, path_map: &mut HashMap<Position, i64>) -> Option<i64>{
    if pos.row == row_limit && pos.col == col_limit{
        return Some(score);
    }
    let mut new_positions = VecDeque::new();
    if pos.row + 1 <= row_limit {
        if grid[pos.row+1][pos.col] != '#'{
            new_positions.push_back((Position{row: pos.row+1, col: pos.col}, score+1));
        }
    }
    if pos.col + 1 <= col_limit {
        if grid[pos.row][pos.col+1] != '#'{
            new_positions.push_back((Position{row: pos.row, col: pos.col+1}, score+1));
        }
    }
    if pos.row as i32 - 1 >= 0 {
        if grid[pos.row-1][pos.col] != '#'{
            new_positions.push_back((Position{row: pos.row-1, col: pos.col}, score+1));
        }
    }
    if pos.col as i32 - 1 >= 0 {
        if grid[pos.row][pos.col-1] != '#'{
            new_positions.push_back((Position{row: pos.row, col: pos.col-1}, score+1));
        }
    }

    while let Some((new_pos, new_score)) = new_positions.pop_front(){
        //println!("new_pos: {:?}{}", new_pos, new_score);
        if new_pos.row == row_limit && new_pos.col == col_limit { return Some(new_score) }
        let mut positions_around = vec![];
        if new_pos.row + 1 <= row_limit {
            if grid[new_pos.row+1][new_pos.col] != '#'{
                positions_around.push(Position{row: new_pos.row+1, col: new_pos.col});
            }
        }
        if new_pos.col + 1 <= col_limit {
            if grid[new_pos.row][new_pos.col+1] != '#'{
                positions_around.push(Position{row: new_pos.row, col: new_pos.col+1});
            }
        }
        if new_pos.row as i32 - 1 >= 0 {
            if grid[new_pos.row-1][new_pos.col] != '#'{
                positions_around.push(Position{row: new_pos.row-1, col: new_pos.col});
            }
        }
        if new_pos.col as i32 - 1 >= 0 {
            if grid[new_pos.row][new_pos.col-1] != '#'{
                positions_around.push(Position{row: new_pos.row, col: new_pos.col-1});
            }
        }
        for pos_around in &positions_around{
            if !path_map.contains_key(pos_around){
                path_map.insert(pos_around.clone(), new_score+1);
                new_positions.push_back((pos_around.clone(), new_score+1));
            }
            else if path_map[&pos_around] > new_score+1{
                path_map.insert(pos_around.clone(), new_score+1);
                new_positions.push_back((pos_around.clone(), new_score+1));
            }
        }
    }
    return None
}