use std::io;

fn main() {
    // Every value in Rust is of a certain data type, which tells Rust 
    // what kind of data is being specified so it knows how to work 
    // with that data. We’ll look at two data type subsets: scalar and 
    // compound.

    // Keep in mind that Rust is a statically typed language, which 
    // means that it must know the types of all variables at compile 
    // time. The compiler can usually infer what type we want to use 
    // sbased on the value and how we use it. In cases when many types 
    // are possible, we must add a type annotation, like this:

    let guess: u32 = "42".parse().expect("Not a number!");

    // If we don’t add the : u32 type annotation shown in the preceding
    // code, Rust will display the following error, which means the 
    // compiler needs more information from us to know which type we 
    // want to use.

    // let guess = "42".parse().expect("Not a number!");
    println!("Guess: {guess}");

    // SCALAR TYPES

    // A scalar type represents a single value. Rust has four primary 
    // scalar types: integers, floating-point numbers, Booleans, and 
    // characters. You may recognize these from other programming 
    // languages.

    // SCALAR TYPES: Integer Types

    // An integer is a number without a fractional component. This type
    // declaration indicates that the value it’s associated with should
    // be an unsigned integer (signed integer types start with i 
    // instead of u) that takes up 32 bits of space. Table 3-1 shows 
    // the built-in integer types in Rust. We can use any of these 
    // variants to declare the type of an integer value.

    // Length	Signed	Unsigned
    // -------------------------
    // 8-bit	i8      u8
    // 16-bit	i16     u16
    // 32-bit	i32     u32
    // 64-bit	i64     u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    // Each variant can be either signed or unsigned and has an 
    // explicit size. Signed and unsigned refer to whether it’s 
    // possible for the number to be negative—in other words, whether 
    // the number needs to have a sign with it (signed) or whether it 
    // will only ever be positive and can therefore be represented 
    // without a sign (unsigned). It’s like writing numbers on paper: 
    // when the sign matters, a number is shown with a plus sign or a 
    // minus sign; however, when it’s safe to assume the number is 
    // positive, it’s shown with no sign. Signed numbers are stored 
    // using two’s complement representation.

    // Each signed variant can store numbers from -(2^(n - 1)) to 
    // 2^(n - 1) - 1 inclusive, where n is the number of bits that 
    // variant uses. So an i8 can store numbers from -(2^7) to 
    // 2^(7) - 1, which equals -128 to 127. Unsigned variants can store
    // numbers from 0 to 2^(n) - 1, so a u8 can store numbers from 0 to
    // 2^(8) - 1, which equals 0 to 255.

    // Additionally, the isize and usize types depend on the 
    // architecture of the computer your program is running on, which 
    // is denoted in the table as “arch”: 64 bits if you’re on a 64-bit
    // architecture and 32 bits if you’re on a 32-bit architecture.

    // You can write integer literals in any of the forms shown in 
    // Table 3-2. Note that number literals that can be multiple 
    // numeric types allow a type suffix, such as 57u8, to designate 
    // the type. Number literals can also use _ as a visual separator 
    // to make the number easier to read, such as 1_000, which will 
    // have the same value as if you had specified 1000.

    // Number literals      Example
    // --------------------------------
    // Decimal              98_222
    // Hex                  0xff
    // Octal                0o77
    // Binary               0b1111_0000
    // Byte (u8 only)	    b'A'

    // So how do you know which type of integer to use? If you’re 
    // unsure, Rust’s defaults are generally good places to start: 
    // integer types default to i32. The primary situation in which 
    // you’d use isize or usize is when indexing some sort of 
    // collection.

    // SCALAR TYPES: Integer Types - Integer Overflow

    // Let’s say you have a variable of type u8 that can hold values 
    // between 0 and 255. If you try to change the variable to a value 
    // outside that range, such as 256, integer overflow will occur, 
    // which can result in one of two behaviors. When you’re compiling 
    // in debug mode, Rust includes checks for integer overflow that 
    // cause your program to panic at runtime if this behavior occurs. 
    // Rust uses the term panicking when a program exits with an error;

    // When you’re compiling in release mode with the --release flag, 
    // Rust does not include checks for integer overflow that cause 
    // panics. Instead, if overflow occurs, Rust performs two’s 
    // complement wrapping. In short, values greater than the maximum 
    // value the type can hold “wrap around” to the minimum of the 
    // values the type can hold. In the case of a u8, the value 256 
    // becomes 0, the value 257 becomes 1, and so on. The program won’t
    // panic, but the variable will have a value that probably isn’t 
    // what you were expecting it to have. Relying on integer 
    // overflow’s wrapping behavior is considered an error.

    // To explicitly handle the possibility of overflow, you can use 
    // these families of methods provided by the standard library for 
    // primitive numeric types:
    // -> Wrap in all modes with the wrapping_* methods, such as 
    //  wrapping_add. 
    // -> Return the None value if there is overflow with the checked_* 
    //  methods.
    // -> Return the value and a boolean indicating whether there was 
    //  overflow with the overflowing_* methods.
    // -> Saturate at the value’s minimum or maximum values with the 
    //  saturating_* methods.

    // SCALAR TYPES: Floating-Point Types

    // Rust also has two primitive types for floating-point numbers, 
    // which are numbers with decimal points. Rust’s floating-point 
    // types are f32 and f64, which are 32 bits and 64 bits in size, 
    // respectively. The default type is f64 because on modern CPUs, 
    // it’s roughly the same speed as f32 but is capable of more 
    // precision. All floating-point types are signed.

    // Floating-point numbers are represented according to the IEEE-754
    //s standard. The f32 type is a single-precision float, and f64 has
    // double precision.

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x: {x}");
    println!("y: {y}");

    // SCALAR TYPES: Numeric Options

    // Rust supports the basic mathematical operations you’d expect for
    // all the number types: addition, subtraction, multiplication, 
    // division, and remainder. Integer division truncates toward zero 
    // to the nearest integer.

    // Each expression in these statements uses a mathematical operator
    // and evaluates to a single value, which is then bound to a 
    // variable.

    // addition
    let sum = 5 + 10;
    println!("sum: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product: {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("quotient: {quotient}");
    println!("truncated: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");

    // SCALAR TYPES: The Boolean Type

    // As in most other programming languages, a Boolean type in Rust 
    // has two possible values: true and false. Booleans are one byte 
    // in size. The Boolean type in Rust is specified using bool.

    // The main way to use Boolean values is through conditionals, such
    // as an if expression.

    let t = true;
    println!("t: {t}");

    let f: bool = false; // with explicit type annotation
    println!("f: {f}");

    // SCALAR TYPES: The Character Type

    // Rust’s char type is the language’s most primitive alphabetic 
    // type.

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {c}");
    println!("z: {z}");
    println!("heart_eyed_cat: {heart_eyed_cat}");

    // Note that we specify char literals with single quotes, as 
    // opposed to string literals, which use double quotes. Rust’s char
    // type is four bytes in size and represents a Unicode Scalar 
    // Value, which means it can represent a lot more than just ASCII. 
    // Accented letters; Chinese, Japanese, and Korean characters; 
    // emoji; and zero-width spaces are all valid char values in Rust. 
    // Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to 
    // U+10FFFF inclusive. However, a “character” isn’t really a 
    // concept in Unicode, so your human intuition for what a 
    // “character” is may not match up with what a char is in Rust. 

    // COMPOUND TYPES

    // Compound types can group multiple values into one type. Rust has
    // two primitive compound types: tuples and arrays.

    // COMPOUND TYPES: The Tuple Type

    // A tuple is a general way of grouping together a number of values
    // with a variety of types into one compound type. Tuples have a 
    // fixed length: once declared, they cannot grow or shrink in size.

    // We create a tuple by writing a comma-separated list of values 
    // inside parentheses. Each position in the tuple has a type, and 
    // the types of the different values in the tuple don’t have to be 
    // the same.

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // println!("tup: {tup}"); // Cannot print because default formatter does not implement tuples (at least in this fashion)
    println!("tup: {:?}", tup); // Use this instead

    // The variable tup binds to the entire tuple because a tuple is 
    // considered a single compound element. To get the individual 
    // values out of a tuple, we can use pattern matching to 
    // destructure a tuple value,

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {x}");
    println!("The value of y is: {y}");
    println!("The value of y is: {z}");
    
    // This program first creates a tuple and binds it to the variable 
    // tup. It then uses a pattern with let to take tup and turn it 
    // into three separate variables, x, y, and z. This is called 
    // destructuring because it breaks the single tuple into three 
    // parts. Finally, the program prints the value of y, which is 6.4.

    // We can also access a tuple element directly by using a period 
    // (.) followed by the index of the value we want to access.

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    println!("five_hundred: {five_hundred}");

    let six_point_four = x.1;
    println!("six_point_four: {six_point_four}");

    let one = x.2;
    println!("one: {one}");

    // This program creates the tuple x and then accesses each element 
    // of the tuple using their respective indices. As with most 
    // programming languages, the first index in a tuple is 0.

    // The tuple without any values has a special name, unit. This 
    // value and its corresponding type are both written () and 
    // represent an empty value or an empty return type. Expressions 
    // implicitly return the unit value if they don’t return any other 
    // value.

    // COMPOUND TYPES: The Array Type

    // Another way to have a collection of multiple values is with an 
    // array. Unlike a tuple, every element of an array must have the 
    // same type. Unlike arrays in some other languages, arrays in Rust
    // have a fixed length.

    // We write the values in an array as a comma-separated list inside
    // square brackets.

    let a = [1, 2, 3, 4, 5];
    // println!("a: {a}"); // Cannot print because default formatter does not implement arrays (at least in this fashion)
    println!("a: {:?}", a); // Use this instead

    // Arrays are useful when you want your data allocated on the stack
    // rather than the heap or when you want to ensure you always have 
    // a fixed number of elements. An array isn’t as flexible as the 
    // vector type, though. A vector is a similar collection type 
    // provided by the standard library that is allowed to grow or 
    // shrink in size.

    // However, arrays are more useful when you know the number of 
    // elements will not need to change.

    let months = [
        "January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"
    ];
    println!("months: {:?}", months);

    // You write an array’s type using square brackets with the type of
    // each element, a semicolon, and then the number of elements in 
    // the array.

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);

    // Here, i32 is the type of each element. After the semicolon, the 
    // number 5 indicates the array contains five elements.

    // You can also initialize an array to contain the same value for 
    // each element by specifying the initial value, followed by a 
    // semicolon, and then the length of the array in square brackets.

    let a = [3; 5];
    println!("a: {:?}", a);

    // The array named a will contain 5 elements that will all be set 
    // to the value 3 initially. This is the same as writing let a = 
    // [3, 3, 3, 3, 3]; but in a more concise way.

    // COMPOUND TYPES: The Array Type - Accessing Array Elements

    // An array is a single chunk of memory of a known, fixed size that
    // can be allocated on the stack. You can access elements of an 
    // array using indexing, like this:

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("a: {:?}", a);
    println!("a first: {first}");
    println!("a second: {second}");

    // In this example, the variable named first will get the value 1 
    // because that is the value at index [0] in the array. The 
    // variable named second will get the value 2 from index [1] in the
    // array.

    // COMPOUND TYPES: The Array Type - Invalid Array Element Access

    // Let’s see what happens if you try to access an element of an 
    // array that is past the end of the array.

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    // This code compiles successfully. If you run this code using 
    // cargo run and enter 0, 1, 2, 3, or 4, the program will print out
    // the corresponding value at that index in the array. If you 
    // instead enter a number past the end of the array, such as 10, 
    // you’ll see output like this:

    // thread 'main' panicked at src/main.rs:19:19:
    // index out of bounds: the len is 5 but the index is 10
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // The program resulted in a runtime error at the point of using 
    // an invalid value in the indexing operation. The program exited 
    // with an error message and didn’t execute the final println! 
    // statement. When you attempt to access an element using indexing, 
    // Rust will check that the index you’ve specified is less than the
    // array length. If the index is greater than or equal to the 
    // length, Rust will panic. This check has to happen at runtime, 
    // especially in this case, because the compiler can’t possibly 
    // know what value a user will enter when they run the code later.

    // This is an example of Rust’s memory safety principles in action.
    // In many low-level languages, this kind of check is not done, and
    // when you provide an incorrect index, invalid memory can be 
    // accessed. Rust protects you against this kind of error by 
    // immediately exiting instead of allowing the memory access and 
    // continuing.
}
