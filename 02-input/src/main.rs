use std::io;

fn main() {
    // Print a prompt to the console
    println!("Please enter some text:");

    // Create a mutable String variable to store the user's input
    let mut input = String::new();

    // Read the user's input from the console and store it in the input variable
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Print the user's input to the console
    println!("You entered: {}", input);
}
