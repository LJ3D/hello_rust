pub mod fibonacci_sequence{
    use std::io;

    pub fn calculate_fibonacci_sequence(){
        println!("===========================");
        println!("==== Fibonacci sequence ===");
        println!("===========================");

        // 1: Get user input for number of iterations to calculate
        println!("Enter the number of iterations to calculate:");
        let mut n_iterations = String::new();
        io::stdin().read_line(&mut n_iterations)
            .expect("Failed to read line");
        let n_iterations_int: u32 = match n_iterations.trim().parse(){
            Ok(n_iterations_int) => n_iterations_int,
            Err(_) => {
                println!("Not a valid number! Defaultuing to 8");
                8 // Default to 8 iterations
            },
        };

        // 2: Calculate the fibonacci sequence!
        let mut previous_two_numbers = (1, 1);
        let mut results: Vec<u128> = Vec::new();
        results.push(1);
        for _i in 0..n_iterations_int{
            previous_two_numbers = (previous_two_numbers.1, previous_two_numbers.0 + previous_two_numbers.1);
            results.push(previous_two_numbers.0);
        }

        // 3: Print the results
        println!("The fibonacci sequence is:");
        for i in results{
            println!("{}", i);
        }   
    }
}