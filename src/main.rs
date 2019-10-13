// import the io (input/output) library from the standard library (std)
use std::io;

// to declare a function, we use the fn keyword followed by the function name
fn main() {
    // println! prints a value to the terminal
    println!("Guess the number!");
    println!("Please input your guess.");
    // Rust variables are immutable by default.
    // To declare a mutable variable, we use the "mut" keyword
    // "::" syntax indicates that "new" is an associated function of the String type
    // String::new() creates a new empty string
    let mut guess = String::new();
    // this calls the read_line method from the standard library to get input from the user
    // read_line takes a string variable as an argument
    // the "&" symbol indicates that the passed in argument is a reference
    // references give a way to let multiple parts of your code access one piece of data without needing
    // to copy that data into memory multiple places.
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line"); // if the instance of io::Result is an Err value, this will cause the program to crash and display the passed argument
    println!("You guessed: {}", guess ); // the set of curly brackets serve as a variable placeholder
}
