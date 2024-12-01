use std::fs;

fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let content_vec: Vec<&str> = file_contents.lines().collect();

    let mut first_numbers: Vec<i32> = Vec::new();
    let mut second_numbers: Vec<i32> = Vec::new();

    for row in content_vec {
        let rowparts: Vec<&str> = row.split_whitespace().collect();
        if let (Some(first), Some(second)) = (rowparts.get(0), rowparts.get(1)) {
            first_numbers.push(first.parse::<i32>().expect("Parsing went wrong"));
            second_numbers.push(second.parse::<i32>().expect("Parsing went wrong"));
        }
    }
    first_numbers.sort();
    second_numbers.sort();
    let list_length = first_numbers.len();


    //part 1
    let mut difference_of_numbers: Vec<i32> = Vec::new();
    for n in 0..list_length{
        difference_of_numbers.push((first_numbers[n] - second_numbers[n]).abs());
    }
    let sum_of_difference: i32 = difference_of_numbers.iter().sum();
    println!("{}", sum_of_difference);

    // part 2
    let mut total_count = 0;
    for n in 0..list_length{
        let mut count_in_second = 0;
        for m in 0..list_length{
            if first_numbers[n] == second_numbers[m]{
                count_in_second += 1;
            }
        }
        total_count += first_numbers[n] * count_in_second;
    }

    println!("{}", total_count);

}
