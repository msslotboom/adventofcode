use std::fs;
use std::collections::{HashMap, HashSet};

fn is_oob(row_index: i32, col_index: i32, row_amount: &i32, col_amount: &i32) -> bool {
    if row_index < 0 || col_index < 0{
        return true
    }
    else if row_index >= *row_amount || col_index >= *col_amount {
        return true
    }
    return false
}
fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let mut content_vec: Vec<Vec<char>> = file_contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut coords: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (row_index, row) in content_vec.iter().enumerate(){
        for (col_index, character) in row.iter().enumerate(){
            if character != &'.'{
                coords.entry(*character).or_insert_with(Vec::new).push((row_index, col_index));
            }
        }
    }
    let row_amount: i32 = content_vec.len() as i32;
    let col_amount: i32 = content_vec[0].len() as i32;
    let mut antinodes = HashSet::new();
    for (_character, coordinates) in &coords{
        for c1 in coordinates{
            for c2 in coordinates{
                if c1 == c2 {
                    continue
                }
                let dist = (c1.0 as i32 - c2.0 as i32, c1.1 as i32 - c2.1 as i32);
                let mut m = 1;
                loop{
                    let p = (c2.0 as i32 + dist.0*m as i32, c2.1 as i32 + dist.1*m as i32);
                    if !is_oob(p.0 as i32, p.1 as i32, &row_amount, &col_amount){
                        content_vec[p.0 as usize][p.1 as usize] = '#';
                        antinodes.insert(p);
                        m += 1;
                    }
                    else{
                        break;
                    }
                }
                m = -1;
                loop{
                    let p = (c2.0 as i32 + dist.0*m as i32, c2.1 as i32 + dist.1*m as i32);
                    if !is_oob(p.0 as i32, p.1 as i32, &row_amount, &col_amount){
                        content_vec[p.0 as usize][p.1 as usize] = '#';
                        antinodes.insert(p);
                        m -= 1;
                    }
                    else{
                        break;
                    }
                }
            }
        }
    }
    println!("{}", antinodes.len())

}
 