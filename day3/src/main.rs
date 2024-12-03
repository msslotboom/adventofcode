use regex::Regex;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let content_vec: Vec<&str> = file_contents.lines().collect();
    let re = Regex::new(r"(mul\((\d+),(\d+)\))").unwrap();
    let mut counter = 0;
    for row in &content_vec {
        let results: Vec<i32> = re
            .captures_iter(row)
            .filter_map(|capture| {
                let num1 = capture.get(2)?.as_str().parse::<i32>().ok()?;
                let num2 = capture.get(3)?.as_str().parse::<i32>().ok()?;
                return Some(num1 * num2);
            })
            .collect();
        counter += results.into_iter().sum::<i32>();
    }
    println!("{}", counter);
}
