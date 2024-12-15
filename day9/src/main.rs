use std::fs;

fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let numbers: Vec<i64> = file_contents
        .chars() 
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect(); 

    let mut char_seq = String::new();
    let mut num_val: u64 = 0;
    let mut id_vec = vec![];
    for i in 0..numbers.len(){
        if i%2 == 0{
            for _ in 0..numbers[i] { 
                char_seq.push_str(&("f")); 
            }
            while char_seq.len() != id_vec.len(){
                id_vec.push(num_val)
            }
            num_val += 1;
        }
        else{
            for _ in 0..numbers[i] { char_seq.push_str(&(".".to_string())) }
        }
    }
    part1(&char_seq, &id_vec);
}

fn part2(char_seq: &String, id_vec: &Vec<u64>){
    
}

fn part1(char_seq: &String, id_vec: &Vec<u64>){
    let mut id_sum: u64 = 0;
    let stop_index = char_seq.len() - char_seq.chars().filter(|&c| c == '.').count();
    let mut bw_pointer = char_seq.len();
    'outer: for i in 0..stop_index{
        if char_seq.chars().nth(i).unwrap() == '.'{
            bw_pointer -= 1;
            while char_seq.chars().nth(bw_pointer).unwrap() == '.' {
                bw_pointer -= 1;
                if bw_pointer <= i {
                    break 'outer;
                }
            }
            id_sum += i as u64 * id_vec[bw_pointer];
        }
        else{
            id_sum += i as u64 * id_vec[i];
        }
    };
    println!("{}", id_sum)
}
