pub mod prime_number_checker{
    use std::io;

    fn is_integer_prime(x: u32) -> bool{
        if x <= 1{
            return false;
        }
        for i in 2..x{
            if x%i==0{
                return false;
            }
        }
        return true;
    }

    pub fn check_prime_number(){
        println!("===========================");
        println!("== Check if num is prime ==");
        println!("===========================");

        // 1: Get input from user
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let input_int: u32 = match input.trim().parse(){
            Ok(input_int) => input_int,
            Err(_) => {
                println!("Not a valid number! Defaulting to 8");
                8
            }
        };

        // 2: Use above function to check if prime
        if is_integer_prime(input_int){
            println!("Number is a prime!");
        }else{
            println!("No.");
        }

        // Wait for the user to press enter before returning to the main menu
        println!("Press enter to return to the main menu.");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
    }
}