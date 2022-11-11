use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    let mut attempts: u32 = 0;
    println!("==== Guess the number! ====");
    loop{
        println!("Input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess_int: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            },
        };
        println!("You guessed: {}", guess_int);
        attempts += 1;
        match guess_int.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("You took {} attempts.", attempts);
}
