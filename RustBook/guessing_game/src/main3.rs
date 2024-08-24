use std::io;
use rand::Rng;

// Bring a type called std::cmp::Ordering into scope from the standard 
// library. The Ordering type is another enum and has the variants 
// Less, Greater, and Equal. These are the three outcomes that are 
// possible when you compare two values.
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Generate the secret number (between 1 and 100).
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    // We create a mutable (mut) string variable to store our guesses.
    let mut guess = String::new();

    // This let's us read in user input and assign it to the guess 
    // variable.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Rust has a strong, static type system. However, it also has type
    // inference. When we wrote let mut guess = String::new(), Rust was
    // able to infer that guess should be a String and didn’t make us 
    // write the type. The secret_number, on the other hand, is a 
    // number type. A few of Rust’s number types can have a value 
    // between 1 and 100: i32, a 32-bit number; u32, an unsigned 32-bit
    // number; i64, a 64-bit number; as well as others. Unless 
    // otherwise specified, Rust defaults to an i32, which is the type 
    // of secret_number unless you add type information elsewhere that 
    // would cause Rust to infer a different numerical type. Rust 
    // cannot compare a string and a number type.

    // Ultimately, we want to convert the String the program reads as 
    // input into a number type so we can compare it numerically to the
    // secret number.
    let guess: u32 = guess.trim()
        .parse()
        .expect("Please type a number!");

    // Rust allows us to shadow the previous value of guess with a new 
    // one. Shadowing lets us reuse the guess variable name rather than
    // forcing us to create two unique variables, such as guess_str and
    // guess, for example. This feature is often used when you want to 
    // convert a value from one type to another type.

    // We bind this new variable to the expression 
    // guess.trim().parse(). The guess in the expression refers to the 
    // original guess variable that contained the input as a string. 
    // The trim method on a String instance will eliminate any 
    // whitespace at the beginning and end, which we must do to be able
    // to compare the string to the u32, which can only contain 
    // numerical data. The user must press enter to satisfy read_line 
    // and input their guess, which adds a newline character to the 
    // string. For example, if the user types 5 and presses enter, 
    // guess looks like this: 5\n. The \n represents “newline.” (On 
    // Windows, pressing enter results in a carriage return and a 
    // newline, \r\n.) The trim method eliminates \n or \r\n, 
    // resulting in just 5.

    // The parse method on strings converts a string to another type. 
    // Here, we use it to convert from a string to a number. We need to
    // tell Rust the exact number type we want by using let guess: u32.
    // The colon (:) after guess tells Rust we’ll annotate the 
    // variable’s type. Rust has a few built-in number types; the u32 
    // seen here is an unsigned, 32-bit integer. It’s a good default 
    // choice for a small positive number.

    // Additionally, the u32 annotation in this example program and the
    // comparison with secret_number means Rust will infer that 
    // secret_number should be a u32 as well. So now the comparison 
    // will be between two values of the same type!

    // The parse method will only work on characters that can logically
    // be converted into numbers and so can easily cause errors. 
    // Because it might fail, the parse method returns a Result type, 
    // much as the read_line method does. We’ll treat this Result the 
    // same way by using the expect method again. If parse returns an 
    // Err Result variant because it couldn’t create a number from the 
    // string, the expect call will crash the game and print the 
    // message we give it. If parse can successfully convert the string
    // to a number, it will return the Ok variant of Result, and expect
    // will return the number that we want from the Ok value.

    // This line prints the string that now contains the user’s input. 
    println!("You guessed: {}", guess);

    // Compare the secret number to the guess. Output a string to 
    // terminal based on whether the value is too large, too small, or
    // exactly the same.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    // The cmp method compares two values and can be called on anything 
    // that can be comp. Then it returns a variant of the Ordering enum
    // we brought into scope with the use statement. We use a match 
    // expression to decide what to do next based on which variant of 
    // Ordering was returned from the call to cmp with the values in 
    // guess and secret_number.

    // A match expression is made up of arms. An arm consists of a 
    // pattern to match against, and the code that should be run if the
    // value given to match fits that arm’s pattern. Rust takes the 
    // value given to match and looks through each arm’s pattern in 
    // turn. Patterns and the match construct are powerful Rust 
    // features: they let you express a variety of situations your code
    // might encounter and they make sure you handle them all.
}