fn main() {
    // VARIABLES

    // By default, variables are immutable. This is one of many nudges 
    // Rust gives you to write your code in a way that takes advantage 
    // of the safety and easy concurrency that Rust offers. However, 
    // you still have the option to make your variables mutable. Let’s 
    // explore how and why Rust encourages you to favor immutability 
    // and why sometimes you might want to opt out.

    // When a variable is immutable, once a value is bound to a name, 
    // you can’t change that value.
    // let x = 5;
    // println!("The value of x is: {x}");

    // x = 6; // <- Will cause compiler to throw an error because we are reassigning the (immutable) variable.
    // println!("The value of x is: {x}");

    // Compiler errors can be frustrating, but really they only mean 
    // your program isn’t safely doing what you want it to do yet.

    // It’s important that we get compile-time errors when we attempt 
    // to change a value that’s designated as immutable because this 
    // very situation can lead to bugs. If one part of our code 
    // operates on the assumption that a value will never change and 
    // another part of our code changes that value, it’s possible that 
    // the first part of the code won’t do what it was designed to do. 
    // The cause of this kind of bug can be difficult to track down 
    // after the fact, especially when the second piece of code changes
    // the value only sometimes. The Rust compiler guarantees that when
    // you state that a value won’t change, it really won’t change, so 
    // you don’t have to keep track of it yourself. Your code is thus 
    // easier to reason through.

    // But mutability can be very useful, and can make code more 
    // convenient to write. Although variables are immutable by 
    // default, you can make them mutable by adding mut in front of the
    // variable name.
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // CONSTANTS

    // Like immutable variables, constants are values that are bound to
    // a name and are not allowed to change, but there are a few 
    // differences between constants and variables.

    // First, you aren’t allowed to use mut with constants. Constants 
    // aren’t just immutable by default—they’re always immutable. You 
    // declare constants using the const keyword instead of the let 
    // keyword, and the type of the value must be annotated. Know that 
    // you must always annotate the type.

    // Constants can be declared in any scope, including the global 
    // scope, which makes them useful for values that many parts of 
    // code need to know about.

    // The last difference is that constants may be set only to a 
    // constant expression, not the result of a value that could only 
    // be computed at runtime.

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Rust’s naming convention for constants is to use all uppercase 
    // with underscores between words. The compiler is able to evaluate
    // a limited set of operations at compile time, which lets us 
    // choose to write out this value in a way that’s easier to 
    // understand and verify, rather than setting this constant to the 
    // value 10,800.

    // Constants are valid for the entire time a program runs, within 
    // the scope in which they were declared. This property makes 
    // constants useful for values in your application domain that 
    // multiple parts of the program might need to know about, such as 
    // the maximum number of points any player of a game is allowed to 
    // earn, or the speed of light.

    // Naming hardcoded values used throughout your program as 
    // constants is useful in conveying the meaning of that value to 
    // future maintainers of the code. It also helps to have only one 
    // place in your code you would need to change if the hardcoded 
    // value needed to be updated in the future.

    println!("3 hours (in seconds): {THREE_HOURS_IN_SECONDS}");

    // SHADOWING

    // You can declare a new variable with the same name as a previous 
    // variable. Rustaceans say that the first variable is shadowed by 
    // the second, which means that the second variable is what the 
    // compiler will see when you use the name of the variable. In 
    // effect, the second variable overshadows the first, taking any 
    // uses of the variable name to itself until either it itself is 
    // shadowed or the scope ends. We can shadow a variable by using 
    // the same variable’s name and repeating the use of the let 
    // keyword as follows:
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // This program first binds y to a value of 5. Then it creates a 
    // new variable y by repeating let y =, taking the original value 
    // and adding 1 so the value of y is then 6. Then, within an inner 
    // scope created with the curly brackets, the third let statement 
    // also shadows y and creates a new variable, multiplying the 
    // previous value by 2 to give y a value of 12. When that scope is 
    // over, the inner shadowing ends and y returns to being 6. 

    // Shadowing is different from marking a variable as mut because 
    // we’ll get a compile-time error if we accidentally try to 
    // reassign to this variable without using the let keyword. By 
    // using let, we can perform a few transformations on a value but 
    // have the variable be immutable after those transformations have 
    // been completed.

    // The other difference between mut and shadowing is that because 
    // we’re effectively creating a new variable when we use the let 
    // keyword again, we can change the type of the value but reuse the
    // same name. 
    let spaces = "    ";
    let spaces = spaces.len();

    // The first spaces variable is a string type and the second spaces
    // variable is a number type. Shadowing thus spares us from having 
    // to come up with different names, such as spaces_str and 
    // spaces_num; instead, we can reuse the simpler spaces name.
    // However, if we try to use mut for this, as shown here, we’ll get
    // a compile-time error:

    // let mut spaces = "    ";
    // spaces = spaces.len(); // <- Will cause compiler to throw an error because the mutable variable changes type.

    // The error says we’re not allowed to mutate a variable’s type.
}
