use std::fs;

fn check_correct(test_val: &u64, current: &u64, nums: &[u64]) -> bool {
    let combined_nums: u64 = (current.to_string() + &nums[0].to_string()).parse().unwrap();
    if nums.len() == 1 {
       if current + nums[0] == *test_val {
            return true;
        } else if (*current) * nums[0] == *test_val {
            return true;
        } else if combined_nums == *test_val {
            return true;
        } else {
            return false;
        }
    } else {
        let next_nums = if nums.len() == 1 {
            &nums[..]
        } else {
            &nums[1..]
        };
        return check_correct(test_val, &(current + nums[0]), next_nums)
            || check_correct(test_val, &(current * nums[0]), next_nums)
            || check_correct(test_val, &combined_nums, next_nums);
    }
}

fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let content_vec: Vec<&str> = file_contents.lines().collect();
    let mut counter = 0;
    for row in content_vec {
        let split_row: Vec<_> = row.split(":").collect();
        let test_val: u64 = split_row[0].parse().unwrap();
        let nums: Vec<u64> = split_row[1]
            .trim()
            .split(" ")
            .map(|x| -> u64 { x.parse().unwrap() })
            .collect();
        let next_nums = if nums.len() == 1 {
            &nums[..]
        } else {
            &nums[1..]
        };
        if check_correct(&test_val, &nums[0], next_nums) {
            counter += test_val
        }
    }
    println!("{}", counter);
}
