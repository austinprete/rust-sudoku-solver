extern crate time;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use time::PreciseTime;

static mut GUESSES_COUNT: u32 = 0;

fn main() {

    let mut parsed_input;

    // Main input loop 
    loop {
        println!("Which sudoku board do you want to solve? (Type a number between 1 and 7)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Couldn't read from stdin");

        let input = input.trim();

        match input.parse() {
            Ok(num) => {
                parsed_input = num;
            }
            Err(_) => continue,
        };

        if 0 < parsed_input && parsed_input <= 7 {
            break;
        }
    }

    // Load board from file into a string
    let mut f = File::open(format!("boards/board{}.txt", parsed_input)).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    // Split board string by whitespace to get individual numbers
    let number_strings = s.split_whitespace();
    let number_strings: Vec<&str> = number_strings.collect();


    // Create numbers array and populate from number_strings
    let mut numbers: [u32; 81] = [0; 81];
    let mut index = 0;
    
    for number in number_strings {
        numbers[index] = number.parse::<u32>().unwrap();
        index += 1;
    }

    // Print original board
    let line_separator = "-----------------";
    println!("{}", line_separator);

    println!("Board {}:\n", parsed_input);
    print_board(&numbers);

    println!("{}", line_separator);

    // Solve puzzle while calculating the time to find a solution
    let start_time = PreciseTime::now();

    let solved = solve_sudoku(&mut numbers, 0);

    let elapsed = start_time.to(PreciseTime::now());

    // Convert solution time from nanoseconds to milliseconds
    let nanoseconds = elapsed.num_nanoseconds()
        .expect("couldn't unwrap return value from num_nanoseconds()");
    let milliseconds = (nanoseconds as f64) / 1000000.0;

    // Print solved board if a solution was found
    println!("Solution:\n");

    if solved {
        print_board(&numbers);
    } else {
        println!("Recursive solver found no solutions.");
    }

    // Print solver statistics 
    println!("{}", line_separator);

    println!("\nSolution time: {}ms", milliseconds);
    unsafe {
        println!("Guesses: {}\n", GUESSES_COUNT);
    }

    println!("{}", line_separator);
}

fn print_board(puzzle: &[u32]) {
    for index in 0..81 {
        print!("{} ", puzzle[index]);
        if index % 9 == 8 {
            println!("");
        }
    }
}

fn solve_sudoku(puzzle: &mut [u32], position: usize) -> bool {
    if position == 80 {
        return verify_puzzle(puzzle);
    }

    if puzzle[position] != 0 {
        return solve_sudoku(puzzle, position + 1);
    }

    for guess in 1..10 {
        unsafe {
            GUESSES_COUNT += 1;
        }

        if !is_valid_guess(puzzle, position, guess) {
            continue;
        }

        puzzle[position] = guess;

        if solve_sudoku(puzzle, position + 1) {
            return true;
        }
    }

    puzzle[position] = 0;

    false
}

fn is_valid_guess(puzzle: &[u32], position: usize, guess: u32) -> bool {
    valid_for_row(puzzle, position, guess) && valid_for_column(puzzle, position, guess) &&
    valid_for_box(puzzle, position, guess)
}

fn valid_for_row(puzzle: &[u32], position: usize, guess: u32) -> bool {
    let row = position_to_row(position as u32);

    let initial_index = row * 9;
    for index in initial_index..initial_index + 9 {
        let index = index as usize;
        if index == position {
            continue;
        }

        if puzzle[index] == guess {
            return false;
        }
    }

    true
}

fn valid_for_column(puzzle: &[u32], position: usize, guess: u32) -> bool {
    let column = position_to_column(position as u32);

    for row in 0..9 {
        let index = (row * 9 + column) as usize;
        if index == position {
            continue;
        }

        if puzzle[index] == guess {
            return false;
        }
    }

    true
}

fn valid_for_box(puzzle: &[u32], position: usize, guess: u32) -> bool {
    let box_number = position_to_box(position as u32);

    let box_row = box_number / 3;
    let box_column = box_number % 3;

    let initial_row = box_row * 3;
    let initial_column = box_column * 3;

    for row in initial_row..(initial_row + 3) {
        for column in initial_column..(initial_column + 3) {
            let index = (row * 9 + column) as usize;
            if position == index {
                continue;
            }

            if puzzle[index] == guess {
                return false;
            }
        }
    }

    true
}

fn verify_puzzle(puzzle: &[u32]) -> bool {
    for index in 0..80 {
        let value = puzzle[index];
        if !(is_valid_guess(puzzle, index, value)) {
            return false;
        }
    }

    true
}

fn position_to_row(position: u32) -> u32 {
    position / 9
}

fn position_to_column(position: u32) -> u32 {
    position % 9
}

fn position_to_box(position: u32) -> u32 {
    let box_row = position_to_row(position) / 3;
    let box_column = position_to_column(position) / 3;

    (box_row * 3) + box_column
}