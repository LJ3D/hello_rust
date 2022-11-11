pub mod reverse_string{
    use std::io;

    pub fn reverse_a_string(){
        println!("===========================");
        println!("===== String reverser =====");
        println!("===========================");
        
        // 1: Get user input
        println!("Input a string to reverse:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        
        // 2: Reverse the string
        let reversed = input.chars().rev().collect::<String>();

        // 3: Print the reversed string
        println!("The reversed string is: {}", reversed);

        // Wait for the user to press enter before returning to the main menu
        println!("Press enter to return to the main menu.");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
    }
}