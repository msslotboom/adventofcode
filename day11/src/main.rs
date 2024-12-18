use std::collections::HashMap;



fn main() {
    let input: Vec<u64> = vec![125, 17];
    let mut stones: HashMap<u64, u64> = HashMap::new();
    for stone in &input{
        stones.insert(*stone, 1);
    }
    for _ in 0..75{
        stones = blink(&stones);
    }
    println!("{:?}", stones.values().sum::<u64>())
}

fn blink(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones: HashMap<u64, u64> = HashMap::new();
    for stone in stones.keys(){
        if *stone == 0 {
            new_stones.entry(1).and_modify(|v| *v += stones[stone]).or_insert(stones[stone]);
        }
        else if stone.to_string().len() % 2 == 0 {
            let stonestr = stone.to_string();
            let val1 = stonestr[..stonestr.len()/2].parse().unwrap();
            let val2 = stonestr[stonestr.len()/2..].parse().unwrap();
            new_stones.entry(val1).and_modify(|v| *v += stones[stone]).or_insert(stones[stone]);
            new_stones.entry(val2).and_modify(|v| *v += stones[stone]).or_insert(stones[stone]);
        }
        else {
            new_stones.entry(stone * 2024).and_modify(|v| *v += stones[stone]).or_insert(stones[stone]);
        }
    }
    return new_stones
}