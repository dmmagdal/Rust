use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Generate the secret number (between 1 and 100).
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Let’s delete the println! that outputs the secret number. That
    // worked well for testing, but it ruins the game.
    println!("The secret number is {secret_number}");

    // The loop keyword creates an infinite loop. The program will now 
    // ask for another guess forever, which actually introduces a new 
    // problem. It doesn’t seem like the user can quit! The user could 
    // always interrupt the program by using the keyboard shortcut 
    // ctrl-c. But there’s another way to escape this insatiable 
    // monster: if the user enters a non-number answer, the program 
    // will crash.
    loop {
        println!("Please input your guess.");
    
        // We create a mutable (mut) string variable to store our guesses.
        let mut guess = String::new();
    
        // This let's us read in user input and assign it to the guess 
        // variable.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // To further refine the game’s behavior, rather than crashing 
        // the program when the user inputs a non-number, let’s make 
        // the game ignore a non-number so the user can continue 
        // guessing. We can do that by altering the line where guess is
        // converted from a String to a u32.
    
        // Convert guess from string to uint32 with shadowing.
        let guess: u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        // We switch from an expect call to a match expression to move 
        // from crashing on an error to handling the error. Remember 
        // that parse returns a Result type and Result is an enum that 
        // has the variants Ok and Err. We’re using a match expression 
        // here, as we did with the Ordering result of the cmp method.

        // If parse is able to successfully turn the string into a 
        // number, it will return an Ok value that contains the 
        // resultant number. That Ok value will match the first arm’s 
        // pattern, and the match expression will just return the num 
        // value that parse produced and put inside the Ok value. That 
        // number will end up right where we want it in the new guess 
        // variable we’re creating.

        // If parse is not able to turn the string into a number, it 
        // will return an Err value that contains more information 
        // about the error. The Err value does not match the Ok(num) 
        // pattern in the first match arm, but it does match the Err(_)
        // pattern in the second arm. The underscore, _, is a catchall 
        // value; in this example, we’re saying we want to match all 
        // Err values, no matter what information they have inside 
        // them. So the program will execute the second arm’s code, 
        // continue, which tells the program to go to the next 
        // iteration of the loop and ask for another guess. So, 
        // effectively, the program ignores all errors that parse might
        // encounter!
    
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

                // Adding the break line after You win! makes the 
                // program exit the loop when the user guesses the 
                // secret number correctly. Exiting the loop also means
                // exiting the program, because the loop is the last 
                // part of main.
            },
        }
    }
}