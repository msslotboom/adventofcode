use regex::Regex;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let content_vec: Vec<&str> = file_contents.lines().collect();
    let re = Regex::new(r"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))").unwrap();
    let mut counter = 0;
    let mut execute_mul = true;
    for row in &content_vec {
        for capture in re.captures_iter(row) {
            if let Some(_do_match) = capture.get(1).filter(|x| x.as_str() == "do()") {
                execute_mul = true;
            } else if let Some(_dont_match) = capture.get(1).filter(|x| x.as_str() == "don't()") {
                execute_mul = false;
            } else if let Some(_) = capture.get(1) {
                if execute_mul {
                    let num1 = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    let num2 = capture.get(3).unwrap().as_str().parse::<i32>().unwrap();
                    counter += num1 * num2;
                }
            }
        }
    }
    println!("{}", counter);
}
