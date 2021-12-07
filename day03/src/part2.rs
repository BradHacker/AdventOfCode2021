use std::fs;
use std::cmp::Ordering;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("couldn't find input.txt");
    let lines: Vec<&str> = input.lines().collect();

    let o2_data = filter_o2_on_bit(0, &lines);
    let co2_data = filter_co2_on_bit(0, &lines);

    let o2_val = isize::from_str_radix(&o2_data, 2).unwrap();
    let co2_val = isize::from_str_radix(&co2_data, 2).unwrap();
    println!("o2 = {}, co2 = {}", o2_val, co2_val);
    println!("Life support rating = {}", o2_val * co2_val);
}

fn filter_o2_on_bit(index: usize, inputs: &[&str]) -> String {
    if inputs.len() == 1 {
        return inputs[0][..].to_owned();
    }
    let mut ones: Vec<&str> = vec!();
    let mut zeros: Vec<&str> = vec!();

    for input in inputs {
        let chars: Vec<char> = input.chars().collect();
        match chars[index] {
                '1' => ones.push(input),
                '0' => zeros.push(input),
                _ => continue,
        }
    }

    match ones.len().cmp(&zeros.len()) {
        Ordering::Less => return filter_o2_on_bit(index + 1, &zeros),
        _ => return filter_o2_on_bit(index + 1, &ones),
    }
}

fn filter_co2_on_bit(index: usize, inputs: &[&str]) -> String {
    if inputs.len() == 1 {
        return inputs[0][..].to_owned();
    }
    let mut ones: Vec<&str> = vec!();
    let mut zeros: Vec<&str> = vec!();

    for input in inputs {
        let chars: Vec<char> = input.chars().collect();
        match chars[index] {
                '1' => ones.push(input),
                '0' => zeros.push(input),
                _ => continue,
        }
    }

    match ones.len().cmp(&zeros.len()) {
        Ordering::Less => return filter_co2_on_bit(index + 1, &ones),
        _ => return filter_co2_on_bit(index + 1, &zeros),
    }
}