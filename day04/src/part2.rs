use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("couldn't find input.txt");
    let mut lines: Vec<&str> = input.lines().collect();

    let bingo_inputs = lines.drain(0..2).next().unwrap().split(',');

    // Load all the boards into the `boards` array
    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut current_board: Vec<Vec<i32>> = Vec::new();
    let mut board_row = 0;
    for line in lines {
        if line.len() == 0 {
            boards.push(current_board.to_owned());
            current_board = Vec::new();
            board_row = 0;
            continue;
        }
        let cols = line.split_whitespace();
        current_board.push(Vec::new());
        for col in cols {
            let num: i32 = col.parse().expect("could not parse into i32");
            current_board[board_row].push(num);
        }
        board_row += 1;
    }
    boards.push(current_board);
    
    // Create all the marker boards
    let mut marker_boards: Vec<Vec<Vec<bool>>> = Vec::new();
    for _ in 0..boards.len() {
        marker_boards.push(vec!(vec!(false; boards[0][0].len()); boards[0].len()))
    }

    let mut end = false;
    let mut boards_won: Vec<usize> = Vec::new();
    let mut last_to_win_board_index = 0;
    let mut winning_num = 0;
    for number in bingo_inputs {
        let number: i32 = number.parse().expect("could not parse number from string");
        for i in 0..boards.len() {
            if boards_won.contains(&i) {
                continue;
            }
            let won = mark_number(boards[i].to_owned(), &mut marker_boards[i], number);
            if won {
                boards_won.push(i)
            }
            if boards_won.len() == boards.len() {
                end = true;
                println!("Winning Board\n-----------");
                print_board(boards[i].to_owned(), marker_boards[i].to_owned());
                last_to_win_board_index = i;
                break;
            }
        }
        if end {
            println!("Winning number is: {}", number);
            winning_num = number;
            break;
        }
    }

    let unmarked_sum = sum_unmarked(boards[last_to_win_board_index].to_owned(), marker_boards[last_to_win_board_index].to_owned());

    println!("Answer is {}", winning_num * unmarked_sum);
}

fn sum_unmarked(board: Vec<Vec<i32>>, markers: Vec<Vec<bool>>) -> i32 {
    let mut sum = 0;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if !markers[i][j] {
                sum += board[i][j];
            }
        }
    }
    sum
}

fn check_win(markers: Vec<Vec<bool>>, i: usize, j: usize) -> bool {
    // check row win
    let mut num_in_row = 0;
    for j in 0..markers[i].len() {
        if markers[i][j] {
            num_in_row += 1;
        }
    }
    if num_in_row == markers[i].len() {
        return true
    }
    
    // check col win
    let mut num_in_col = 0;
    for i in 0..markers.len() {
        if markers[i][j] {
            num_in_col += 1;
        }
    }
    if num_in_col == markers.len() {
        return true
    }
    false
}

fn mark_number(board: Vec<Vec<i32>>, markers: &mut Vec<Vec<bool>>, number: i32) -> bool {
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == number {
                markers[i][j] = true;
                return check_win(markers.to_owned(), i, j);
            }
        }
    }
    false
}

fn print_board(board: Vec<Vec<i32>>, markers: Vec<Vec<bool>>) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if markers[i][j] {
                print!("[{:0>2}]", board[i][j]);
            } else {
                print!(" {:0>2} ", board[i][j]);
            }
        }
        print!("\n");
    }
    print!("\n")
}