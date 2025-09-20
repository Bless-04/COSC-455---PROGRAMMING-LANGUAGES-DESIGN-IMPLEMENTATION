use rand::Rng;
use std::io;
fn main() {
    let N: u8  = rand::rng().random_range(1..=100);
    let mut buffer = String::new();
    let mut guessed_number: u8; // assigning a default value makes a (never read) warning appear

    loop {
        println!("Enter a number between 1 and 100");
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input");

        guessed_number = buffer
            .trim()
            .parse::<u8>()
            .expect("Input is not a valid number");

        if guessed_number == N {
            println!("You guessed the number.");
            break;
        }

        if guessed_number < N {
            println!("{} is too low",guessed_number);
        } else {
            println!("{} is too high",guessed_number);
        }

        buffer.clear();
    }
}