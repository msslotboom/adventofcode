use std::fs;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn update_direction(&self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}
fn char_to_direction(c: char) -> Option<Direction> {
    match c {
        '^' => Some(Direction::Up),
        'v' => Some(Direction::Down),
        '<' => Some(Direction::Left),
        '>' => Some(Direction::Right),
        _ => None, // Return None for characters that don't map to a direction
    }
}

fn direction_to_char(direction: Direction) -> char {
    match direction {
        Direction::Up => '^',
        Direction::Down => 'v',
        Direction::Left => '<',
        Direction::Right => '>',
    }
}

fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let mut content_vec: Vec<Vec<char>> = file_contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut row_nr = 0;
    let mut column_nr = 0;
    let mut found = false;
    for (row_index, row) in content_vec.iter().enumerate() {
        for (col_index, &element) in row.iter().enumerate() {
            if element == '^' || element == 'v' || element == '>' || element == '<' {
                found = true;
                row_nr = row_index;
                column_nr = col_index;
                break;
            }
        }
        if found {
            break;
        }
    }
    let direction = char_to_direction(content_vec[row_nr][column_nr]);
    let mut oob = false;
    let mut counter = 0;
    loop {
        let mut direction = char_to_direction(content_vec[row_nr][column_nr]);
        let mut new_row = row_nr;
        let mut new_col = column_nr;
        match direction {
            Some(Direction::Up) => {
                if row_nr == 0 {
                    oob = true;
                } else {
                    new_row = row_nr - 1;
                }
            }
            Some(Direction::Right) => {
                if column_nr == content_vec[0].len() - 1 {
                    oob = true;
                } else {
                    new_col = column_nr + 1;
                }
            }
            Some(Direction::Down) => {
                if row_nr == content_vec.len() - 1 {
                    oob = true;
                } else {
                    new_row = row_nr + 1;
                }
            }
            Some(Direction::Left) => {
                if column_nr == 0 {
                    oob = true;
                } else {
                    new_col = column_nr - 1;
                }
            }
            None => {
                println!("Something went wrong");
            }
        }
        if oob {
            break;
        }
        if content_vec[new_row][new_col] == '#' {
            direction = Some(
                direction
                    .expect("Direction update went wrong")
                    .update_direction(),
            );
            content_vec[row_nr][column_nr] =
                direction_to_char(direction.expect("Direction is broken"));
            continue;
        } else if content_vec[new_row][new_col] == '.' {
            counter += 1;
        }
        content_vec[row_nr][column_nr] = 'X';
        if row_nr != new_row {
            row_nr = new_row;
        }
        if column_nr != new_col {
            column_nr = new_col
        }
        content_vec[new_row][new_col] = direction_to_char(direction.expect("Direction is broken"));
    }
    println!("{}", counter + 1);
}
