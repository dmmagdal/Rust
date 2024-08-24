// Import io from the standard library. This will let us read user 
// input.
use std::io;

fn main() {
    // fn declares our function (main() in this case) which takes no
    // parameters.
    println!("Guess the number!");

    println!("Please input your guess.");

    // We create a mutable (mut) string variable to store our guesses.
    // Variables are immutable by default in rust. String::new() is a
    // function that returns an instance of a string.
    let mut guess = String::new();

    // String is a string type provided by the standard library that is
    // a growable, UTF-8 encoded bit of text.

    // The :: syntax in the ::new line indicates that new is an 
    // associated function of the String type. An associated function 
    // is a function that’s implemented on a type, in this case String.
    // This new function creates a new, empty string. You’ll find a new
    // function on many types because it’s a common name for a function
    // that makes a new value of some kind.

    // This let's us read in user input and assign it to the guess 
    // variable.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // If we hadn’t imported the io library with use std::io; at the
    // beginning of the program, we could still use the function by
    // writing this function call as std::io::stdin. The stdin function
    // returns an instance of std::io::Stdin, which is a type that 
    // represents a handle to the standard input for your terminal.

    // .read_line(&mut guess) calls the read_line method on the 
    // standard input handle to get input from the user. We’re also 
    // passing &mut guess as the argument to read_line to tell it what 
    // string to store the user input in. The full job of read_line is 
    // to take whatever the user types into standard input and append 
    // that into a string (without overwriting its contents), so we 
    // therefore pass that string as an argument. The string argument 
    // needs to be mutable so the method can change the string’s 
    // content.

    // The & indicates that this argument is a reference, which gives 
    // you a way to let multiple parts of your code access one piece of
    // data without needing to copy that data into memory multiple 
    // times. References are a complex feature, and one of Rust’s major
    // advantages is how safe and easy it is to use references. All you
    // need to know is that, like variables, references are immutable 
    // by default. Hence, you need to write &mut guess rather than 
    // &guess to make it mutable.

    // The next part is this method .expect("Failed to read line"); 
    // which we could have written this code as:
    // io::stdin().read_line(&mut guess).expect("Failed to read line");
    // However, one long line is difficult to read, so it’s best to 
    // divide it. It’s often wise to introduce a newline and other 
    // whitespace to help break up long lines when you call a method 
    // with the .method_name() syntax.

    // As mentioned earlier, read_line puts whatever the user enters 
    // into the string we pass to it, but it also returns a Result 
    // value. Result is an enumeration, often called an enum, which is
    // a type that can be in one of multiple possible states. We call 
    // each possible state a variant.

    // The purpose of these Result types is to encode error-handling 
    // information. Result’s variants are Ok and Err. The Ok variant 
    // indicates the operation was successful, and inside Ok is the 
    // successfully generated value. The Err variant means the 
    // operation failed, and Err contains information about how or why
    // the operation failed.

    // Values of the Result type, like values of any type, have methods
    // defined on them. An instance of Result has an expect method that
    // you can call. If this instance of Result is an Err value, expect
    // will cause the program to crash and display the message that you
    // passed as an argument to expect. If this instance of Result is 
    // an Ok value, expect will take the return value that Ok is 
    // holding and return just that value to you so you can use it.

    // If you don’t call expect, the program will compile, but you’ll 
    // get a warning. The right way to suppress the warning is to 
    // actually write error-handling code, but in our case we just want
    // to crash this program when a problem occurs, so we can use 
    // expect.

    // This line prints the string that now contains the user’s input. 
    // The {} set of curly brackets is a placeholder: think of {} as 
    // little crab pincers that hold a value in place. When printing 
    // the value of a variable, the variable name can go inside the 
    // curly brackets. When printing the result of evaluating an 
    // expression, place empty curly brackets in the format string, 
    // then follow the format string with a comma-separated list of 
    // expressions to print in each empty curly bracket placeholder in 
    // the same order.
    println!("You guessed: {}", guess);
}