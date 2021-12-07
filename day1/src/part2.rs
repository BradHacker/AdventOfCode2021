use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("couldn't find input.txt");

    let depths: Vec<i32> = input.lines().map(|x| x.trim().parse().expect("cannot parse line into integer")).collect();

    let mut window_a = &depths[0..3];
    let mut window_b = &depths[1..4];
    let mut count = 0;

    for index in 2..depths.len()-1 {
        println!("{} | a = {:?}, b = {:?}", index, window_a, window_b);
        let sum_a = window_a[0] + window_a[1] + window_a[2];
        let sum_b = window_b[0] + window_b[1] + window_b[2];
        if sum_b > sum_a {
            count += 1;
        }
        if index + 3 > depths.len() {
            break;
        }
        window_a = &window_b[0..3];
        window_b = &depths[index..index+3];
    }

    println!("Number of increases is {}", count);
}
