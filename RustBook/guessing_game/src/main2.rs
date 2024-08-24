// Import io from the standard library. This will let us read user 
// input.
use std::io;

// Import Rng from rand library. This will let our program randomly
// generate the secret number each game.
use rand::Rng;

fn main() {
    // fn declares our function (main() in this case) which takes no
    // parameters.
    println!("Guess the number!");

    // The Rng trait defines methods that random number generators 
    // implement, and this trait must be in scope for us to use those 
    // methods.

    // We call the rand::thread_rng function that gives us the 
    // particular random number generator we’re going to use: one that 
    // is local to the current thread of execution and is seeded by 
    // the operating system. Then we call the gen_range method on the 
    // random number generator. This method is defined by the Rng trait
    // that we brought into scope with the use rand::Rng; statement. 
    // The gen_range method takes a range expression as an argument and
    // generates a random number in the range. The kind of range 
    // expression we’re using here takes the form start..=end and is 
    // inclusive on the lower and upper bounds, so we need to specify 
    // 1..=100 to request a number between 1 and 100.

    // Each crate has documentation with instructions for using it. 
    // Another neat feature of Cargo is that running the cargo doc 
    // --open command will build documentation provided by all your 
    // dependencies locally and open it in your browser. If you’re 
    // interested in other functionality in the rand crate, for 
    // example, run cargo doc --open and click rand in the sidebar on 
    // the left.

    // Generate the secret number (between 1 and 100).
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    // Remember that a crate is a collection of Rust source code files.
    // The project we’ve been building is a binary crate, which is an 
    // executable. The rand crate is a library crate, which contains 
    // code that is intended to be used in other programs and can’t be
    // executed on its own.

    // We can either modify the Cargo.toml file to include the crate in
    // our [dependencies] OR use `cargo add rand` to add the rand crate
    // to our dependencies (which will automatically update the 
    // Cargo.toml).

    // In the Cargo.toml file, everything that follows a header is part
    // of that section that continues until another section starts. In
    // [dependencies] you tell Cargo which external crates your project
    // depends on and which versions of those crates you require. Cargo
    // understands Semantic Versioning (sometimes called SemVer), which
    // is a standard for writing version numbers. The specifier 0.8.5 
    // is actually shorthand for ^0.8.5, which means any version that 
    // is at least 0.8.5 but below 0.9.0.

    // When we include an external dependency, Cargo fetches the latest
    // versions of everything that dependency needs from the registry, 
    // which is a copy of data from Crates.io. Crates.io is where 
    // people in the Rust ecosystem post their open source Rust 
    // projects for others to use.

    // After updating the registry, Cargo checks the [dependencies] 
    // section and downloads any crates listed that aren’t already 
    // downloaded. After downloading the crates, Rust compiles them and
    // then compiles the project with the dependencies available.

    // If you immediately run cargo build again without making any 
    // changes, you won’t get any output aside from the Finished line. 
    // Cargo knows it has already downloaded and compiled the 
    // dependencies, and you haven’t changed anything about them in 
    // your Cargo.toml file. Cargo also knows that you haven’t changed 
    // anything about your code, so it doesn’t recompile that either. 
    // With nothing to do, it simply exits.

    // Cargo only updates the build with your tiny change to the 
    // src/main.rs file. Your dependencies haven’t changed, so Cargo 
    // knows it can reuse what it has already downloaded and compiled 
    // for those.

    // When you do want to update a crate, Cargo provides the command 
    // update, which will ignore the Cargo.lock file and figure out all
    // the latest versions that fit your specifications in Cargo.toml. 
    // Cargo will then write those versions to the Cargo.lock file. 

    println!("Please input your guess.");

    // We create a mutable (mut) string variable to store our guesses.
    let mut guess = String::new();

    // This let's us read in user input and assign it to the guess 
    // variable.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // This line prints the string that now contains the user’s input. 
    println!("You guessed: {}", guess);
}