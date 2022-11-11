use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Create a random number between 1 and 100, and a variable to store the number of guesses
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    let mut attempts: u32 = 0;
    
    println!("==== Guess the number! ====");
    loop{
        // 1: Get user input
        println!("Input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        // 2: Convert user input to u32
        let guess_int: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            },
        };

        // 3: Print user input
        println!("You guessed: {}", guess_int);
        attempts += 1; // And increment the number of attempts

        // 4: Compare user input to secret number
        match guess_int.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { // If the user guessed correctly, they win!
                println!("You win!");
                break; // Break out of the loop, ending the game
            }
        }
    }
    // Finally, print the number of attempts it took the user to guess the number
    println!("You took {} attempts.", attempts);
}
