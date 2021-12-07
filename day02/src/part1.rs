use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("couldn't find input.txt");

    let instructions = input.lines();

    let mut h_pos = 0;
    let mut depth = 0;

    for instruction in instructions {
        let instruction_parts: Vec<&str> = instruction.split_whitespace().collect();
        let direction = instruction_parts[0];
        let units: i32 = instruction_parts[1].parse().expect("could not parse units");
        match direction {
            "forward" => h_pos += units,
            "up" => depth -= units,
            "down" => depth += units,
            _ => continue,
        }
    }

    let final_pos = h_pos * depth;

    println!("Final position is {}", final_pos);
}
