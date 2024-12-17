fn main() {
    let mut input: Vec<u64> = vec![125, 17];
    for _ in 0..25{
        input = blink(input);
    }
    println!("{:?}", input.len())
}

fn blink(stones: Vec<u64>) -> Vec<u64> {
    let mut new_stones = vec![];
    for stone in stones{
        if stone == 0 {
            new_stones.push(1)
        }
        else if stone.to_string().len() % 2 == 0 {
            let stonestr = stone.to_string();
            new_stones.push(stonestr[..stonestr.len()/2].parse().unwrap());
            new_stones.push(stonestr[stonestr.len()/2..].parse().unwrap());
        }
        else {
            new_stones.push(stone * 2024)
        }
    }
    return new_stones
}