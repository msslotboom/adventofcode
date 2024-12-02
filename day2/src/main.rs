use std::fs;

fn check_error(splits: &Vec<i32>) -> bool{
        let mut previous_value = &splits[0];
        let is_decreasing;
        if splits[0] > splits[1]{
            is_decreasing = true;
        }
        else if splits[0] < splits[1]{
            is_decreasing = false;
        }
        else {
            return true;
        }
        for number in &splits[1..]{
            if previous_value == number || (!is_decreasing && previous_value > number) || (is_decreasing && previous_value < number) || (number - previous_value).abs() > 3 {
                return true;
            }
            previous_value = number;
        }
    return false
}
fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let content_vec: Vec<&str> = file_contents.lines().collect();

    let mut counter = 0;

    for row in content_vec{
        let splits: Vec<i32> = row.split_whitespace().map(|x| -> i32{x.parse().unwrap()}).collect();
        let splits_length = splits.len();
        for i in 0..splits_length{
            let mut splits_copy = splits.clone();
            splits_copy.remove(i);
            if !check_error(&splits_copy){
                counter += 1;
                break;
            }
        }
    }
    println!("{}", counter);
}
