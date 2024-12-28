#[derive(Clone, Copy)]
struct Registers{
    a: u32,
    b: u32,
    c: u32
}

fn combo_to_int(combo_operand: u32, registers: Registers) -> u32 {
    match combo_operand{
        1_u32..=3_u32 => return combo_operand,
        4 => return registers.a,
        5 => return registers.b,
        6 => return registers.c,
        7 => return 0,
        _ => return 0

    }
}

fn adv(operand: u32, registers: &mut Registers){
    let numerator = registers.a;
    let two: u32 = 2;
    let divisor = two.pow(combo_to_int(operand, *registers));
    let result = numerator/divisor;
    registers.a = result;
}

fn bxl(operand: u32, registers: &mut Registers){
    registers.b = registers.b ^ operand;
}

fn bst(operand: u32, registers: &mut Registers){
    let combo_val = combo_to_int(operand, *registers);
    registers.b = combo_val % 8;
}

fn jnz(operand: u32, registers: &mut Registers) -> Option<u32> {
    if registers.a == 0 { return None }
    return Some(operand)
}

fn bxc(operand: u32, registers: &mut Registers){
    registers.b = registers.b ^ registers.c;
}

fn out(operand: u32, registers: &mut Registers) -> u32{
    let combo_val = combo_to_int(operand, *registers);
    return combo_val % 8;
}

fn bdv(operand: u32, registers: &mut Registers){
    let numerator = registers.a;
    let two: u32 = 2;
    let divisor = two.pow(combo_to_int(operand, *registers));
    let result = numerator/divisor;
    registers.b = result;
}

fn cdv(operand: u32, registers: &mut Registers){
    let numerator = registers.a;
    let two: u32 = 2;
    let divisor = two.pow(combo_to_int(operand, *registers));
    let result = numerator/divisor;
    registers.c = result;
}

fn main() {
    let mut registers = Registers{a:0, b:0, c:0};
    let instructions = vec![];
    let mut counter = 0;
    let mut output = vec![];
    loop{
        match instructions[counter]{
            0 => adv(instructions[counter+1], &mut registers),
            1 => bxl(instructions[counter+1], &mut registers),
            2 => bst(instructions[counter+1], &mut registers),
            3 => {
                let new_counter = jnz(instructions[counter+1], &mut registers);
                if new_counter != None {
                    counter = new_counter.unwrap() as usize;
                    continue;
                }
            },
            4 => bxc(instructions[counter+1], &mut registers),
            5 => {output.push(out(instructions[counter+1], &mut registers))},
            6 => bdv(instructions[counter+1], &mut registers),
            7 => cdv(instructions[counter+1], &mut registers),
            _ => println!("Error!")
        }
        counter += 2;
        if counter > instructions.len()-1 { break }
        if output.len() != 0{
            if output[output.len()-1] != instructions[output.len()-1]{
                break;
            }
        }
    }
    println!("{:?}", output);
}
