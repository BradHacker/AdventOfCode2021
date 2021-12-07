use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("couldn't find input.txt");

    let depths: Vec<i32> = input.lines().map(|x| x.trim().parse().expect("cannot parse line into integer")).collect();

    let mut prev_num = depths[0];
    let mut count = 0;

    for num in depths {
        if num > prev_num {
            count += 1;
        }
        prev_num = num;
    }

    println!("Number of increases is {}", count);
}
