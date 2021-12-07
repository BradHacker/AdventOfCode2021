use std::fs;
use std::cmp::Ordering;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("couldn't find input.txt");
    let lines: Vec<&str> = input.lines().collect();

    let mut ones = [0; 12];
    let mut zeros = [0; 12];

    for line in lines {
        for (i, ch) in line.chars().enumerate() {
            match ch {
                '1' => ones[i] += 1,
                '0' => zeros[i] += 1,
                _ => continue,
            }
        }
    }

    let mut gamma_rate: String = "".to_owned();
    let mut epsilon_rate: String = "".to_owned();
    for i in 0..12 {
        match ones[i].cmp(&zeros[i]) {
            Ordering::Greater => {
                gamma_rate.push_str("1");
                epsilon_rate.push_str("0");
            },
            Ordering::Less => {
                gamma_rate.push_str("0");
                epsilon_rate.push_str("1");
            },
            _ => continue,
        }
    }

    let gamma_val = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_val = isize::from_str_radix(&epsilon_rate, 2).unwrap();
    println!("gamma = {}, epsilon = {}", gamma_val, epsilon_val);
    println!("Power consumption = {}", gamma_val * epsilon_val);
}
