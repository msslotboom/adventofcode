use std::collections::HashMap;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let content_vec: Vec<&str> = file_contents.lines().collect();
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut row_index = 0;
    for row in &content_vec {
        row_index += 1;
        if *row == "" {
            break;
        }
        let nums: Vec<i32> = row
            .split("|")
            .map(|x| -> i32 { x.parse().unwrap() })
            .collect();
        rules.entry(nums[1]).or_insert_with(Vec::new).push(nums[0]);
    }
    let mut updates: Vec<Vec<i32>> = vec![];
    for i in row_index..&content_vec.len() - 1 {
        let nums: Vec<i32> = content_vec[i]
            .split(",")
            .map(|x| -> i32 { x.parse().unwrap() })
            .collect();
        updates.push(nums);
    }
    let mut counter = 0;
    for row in updates {
        //println!("{:?}", row);
        let mut correct_row = true;
        for i in 0..row.len() {
            if rules.contains_key(&row[i]) {
                let disallowed_nums = rules.get(&row[i]).unwrap();
                for disallowed_num in disallowed_nums {
                    if row.contains(disallowed_num) {
                        //println!("{}", disallowed_num);
                        if i == 0 {
                            correct_row = false;
                            break;
                        }
                        let mut correct = false;
                        for j in 0..i {
                            if row[j] == *disallowed_num {
                                //println!("This is correct.");
                                correct = true;
                                break;
                            }
                        }
                        if !correct {
                            correct_row = false;
                            break;
                        }
                    }
                }
            }
        }
        if correct_row {
            let middle_index = row.len() / 2;
            counter += row[middle_index];
        }
    }
    println!("{}", counter);
}
