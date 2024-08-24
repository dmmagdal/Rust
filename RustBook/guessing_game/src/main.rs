use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Generate the secret number (between 1 and 100).
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Loop the game until the user enters the correct guess OR quits
    // the program (with CTRL + C).
    loop {
        println!("Please input your guess.");
    
        // We create a mutable (mut) string variable to store our guesses.
        let mut guess = String::new();
    
        // This let's us read in user input and assign it to the guess 
        // variable.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // Convert guess from string to uint32 with shadowing.
        let guess: u32 = match guess
            .trim()
            .parse() {
                // Ignore non-number guesses and repeat the loop for
                // invalid input as part of error handling.
                Ok(num) => num,
                Err(_) => continue,
            };
    
        // This line prints the string that now contains the user’s input. 
        println!("You guessed: {}", guess);
    
        // Compare the secret number to the guess. Output a string to 
        // terminal based on whether the value is too large, too small, or
        // exactly the same.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // Let’s program the game to quit when the user wins by
                // adding a break statement:
                println!("You win!");
                break;
            },
        }
    }
}