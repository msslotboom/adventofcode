use std::fs;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Robot{
    pos: (i32, i32),
    speed: (i32, i32)
}
impl Robot{
    fn update_pos(&mut self){
        self.pos.0 += self.speed.0;
        self.pos.1 += self.speed.1;
        if self.pos.0 >= 11 { self.pos.0 -= 101; };
        if self.pos.0 < 0 { self.pos.0 += 101; }
        if self.pos.1 >= 7 { self.pos.1 -= 103; }
        if self.pos.1 < 0 { self.pos.1 += 103; }
    }
}
fn main() {
    let file_contents = fs::read_to_string("data.txt").unwrap();
    let mut robot_list = vec![];
    for row in file_contents.lines(){
        let split_row = row.split(" ");
        let mut pos= (0, 0);
        let mut speed = (0,0);
        for split in split_row{
            if split.starts_with("p="){
                let parsed_pos = split
                .trim_start_matches("p=")
                .split(',')
                .map(|s| s.parse::<i32>().unwrap()) 
                .collect::<Vec<_>>();
                pos.0 = parsed_pos[0];
                pos.1 = parsed_pos[1];
            }
            else{
                let parsed_speed = split
                .trim_start_matches("v=") 
                .split(',')
                .map(|s| s.parse::<i32>().unwrap()) 
                .collect::<Vec<_>>();
                speed.0 = parsed_speed[0];
                speed.1 = parsed_speed[1];
            }
        }
        let rob = Robot{pos, speed};
        robot_list.push(rob);
    }
    for seconds in 1..100000{
        for robot in &mut robot_list{
            robot.update_pos();
        }
        if seconds % 101 == 13 {
            println!("Iteration: {}", seconds);
            print_grid(&robot_list);
        }
    }
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for robot in robot_list{
        if robot.pos.0 < 50 {
            if robot.pos.1 < 51{
                q1 += 1;
            }
            else if robot.pos.1 > 51{
                q3 += 1;
            }
        }
        else if robot.pos.0 > 50 {
            if robot.pos.1 < 51{
                q2 += 1;
            }
            else if robot.pos.1 > 51{
                q4 += 1;
            }
        }
    }
    let safety_factor = q1 * q2 * q3 * q4;
    println!("{}", safety_factor);
}

fn print_grid(robot_list: &Vec<Robot>){
    for row_index in 0..103{
        let mut printed_row: String = String::new();
        for col_index in 0..101{
            let mut counter = 0;
            for robot in robot_list{
                if robot.pos.0 == col_index && robot.pos.1 == row_index { counter += 1 }
            }
            if counter == 0 { printed_row.push('.')}
            else if counter == 1 { printed_row.push('*') }
            else if counter == 2 { printed_row.push('=') }
            else if counter == 3 { printed_row.push('$') }
            else {printed_row.push('~')}
        }
        println!("{:?}", printed_row);
    }
    println!();
    println!();
    println!();
    println!();

}