use std::io;
use rand::Rng;
fn main() {
    println!("Guess the number game!!");
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    println!("Please input your guess: "); 
    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read user input");
    println!("Your guess is {guess}");
    println!("Secret num is {secret_number}");

}
