mod guessing_game;
use crate::guessing_game::guessing_game::play_guessing_game;
mod reverse_string;
use crate::reverse_string::reverse_string::reverse_a_string;
mod fibonacci_sequence;
use crate::fibonacci_sequence::fibonacci_sequence::calculate_fibonacci_sequence;

use std::io;

fn main() {
    loop{
        // 1: Show the available options
        println!("==== Select some code to run! ====");
        println!("0: Exit");
        println!("1: Guessing game");
        println!("2: Reverse a string");
        println!("3: Fibonacci sequence");
        println!("4: Prime number checker");
        
        // 2: Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        
        // 3: Convert user input to u32
        let input_int: u32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue; // Terminates the current loop iteration and starts the next one
            },
        };

        // 4: Run the selected code based on the user input
        match input_int{
            1 => play_guessing_game(),
            2 => reverse_a_string(),
            3 => calculate_fibonacci_sequence(),
            0 => break,
            _ => println!("Please enter a valid option!"),
        }
    }
    
    // Say goodbye to the user
    println!("Goodbye!");
}