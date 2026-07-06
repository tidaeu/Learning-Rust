use rand::Rng;
use std::io;

fn main() {
    println!("Guess a number!");
    println!("Put your guess here");

    let secret_number = rand::thread_rng().gen_range(0..=100);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");

    println!("Your guess is {guess}, random number generated is {secret_number}");
}
