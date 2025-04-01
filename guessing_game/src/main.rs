use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // Rust 'vars' are immutable by default.  Think const.
    // mut forces mutability.
    let mut guess = String::new();

    // references are also immutable by default
    // the immutable reference to guess would have just been &guess
    // read_line reads into the ref, but it also returns a Result
    // Result is an enum with the variants `Ok` and `Err`
    let size = io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {}, of byte size {}", guess, size);
}
