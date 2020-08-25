/*
To obtain user input and then print the result as output, 
we need to bring the io (input/output) library into scope. 
The io library comes from the standard library (which is known as std):

By default, Rust brings only a few types into the scope of every program in the prelude. 
If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement. 
Using the std::io library provides you with a number of useful features, including the ability to accept user input.
*/
use std::io;

/* The main function is the entry point into the program. */
fn main() {

    // A macro that prints a string to the screen
    println!("Guess the number!");
    println!("Please input your guess.");

    // In Rust, variables are immutable by default.
    // Use mut before the variable name to make a variable mutable.
    // String::new, a function that returns a new instance of a String.
    // The :: syntax in the ::new line indicates that new is an associated function of the String type. 
    // An associated function is implemented on a type, in this case String, rather than on a particular instance of a String. 
    // Some languages call this a static method.
    let mut guess = String::new();

    // If we hadn’t put the use std::io line at the beginning of the program, 
    // we could have written this function call as std::io::stdin. 
    // The stdin function returns an instance of std::io::Stdin, 
    // which is a type that represents a handle to the standard input for your terminal.
    io::stdin()
    // The job of read_line is to take whatever the user types into standard input and place that into a string, 
    // so it takes that string as an argument. 
    // The string argument needs to be mutable so the method can change the string’s content by adding the user input.
    // The & indicates that this argument is a reference, 
    // which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. 
        .read_line(&mut guess)
        .expect("Failed to read line");
        
        //  The set of curly brackets, {}, is a placeholder: think of {} as little crab pincers that hold a value in place. 
        // You can print more than one value using curly brackets: the first set of curly brackets holds the first value 
        // listed after the format string, the second set holds the second value, and so on. 
    println!("You guessed: {}", guess);
}
