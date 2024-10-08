# Cargo Notes

 - Cargo.toml contains cargo's configuration format
     - The `[package]` heading indicates the following section contains statements configuring a package. It contains the name of the package, the package version, and what edition of rust that it is using.
     - The `[dependencies]` heading indicates the following section contains the list of the project's dependencies.
         - In rust, packages are called "crates".
 - The `cargo new` command creates a simple "hello world" program and has placed that program in a `/src` folder.
     - The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. 
     - Using cargo helps you organize your projects. There’s a place for everything, and everything is in its place.
 - Building projects
     - The `cargo build` command creates an executable file in target/debug/hello_cargo (or target\debug\hello_cargo.exe on Windows) rather than in your current directory. 
         - Because the default build is a debug build, Cargo puts the binary in a directory named debug.
         - Running `cargo build` for the first time also causes cargo to create a new file at the top level: Cargo.lock. 
             - This file keeps track of the exact versions of dependencies in your project.
     - We can also use `cargo run` to compile the code and then run the resultant executable all in one command.
     - Cargo figures out when the files haven’t changed, it doesn’t rebuild but just ran the binary. If you had modified your source code, cargo would have rebuilt the project before running it.
     - Cargo also provides a command called `cargo check`. 
         - This command quickly checks your code to make sure it compiles but doesn’t produce an executable.
         - Often, `cargo check` is much faster than `cargo build` because it skips the step of producing an executable. 
             - If you’re continually checking your work while writing the code, using cargo check will speed up the process of letting you know if your project is still compiling.
 - Building for release
     - When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations.
         - The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. 
             - This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible.
 - Cargo as convention
     - Once programs grow to multiple files or need a dependency, it’s much easier to let cargo coordinate the build.