use rand::Rng;
use std::io;
fn main() {

    let N: u8  = rand::rng().random_range(1..=5);       // Create a random number generator

    let mut buffer = String::new();

    while guessed_number != N{

        io::stdin().read_line()


    }


}