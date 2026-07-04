use std::io;

fn main() {
    println!("Guess a number!");
    println!("Put your guess here");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to readline");

    println!("Your guess is {guess}");

}
