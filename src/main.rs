use std::io;

fn main() {
    // Print something
    println!("Hello, I'm start learning Rust :D");

    println!("Guess the number!");

    println!("Please input your guess.");

    // create mutable variable and call String constructor
    let mut guess = String::new();

    // await input form user and put it to variable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
