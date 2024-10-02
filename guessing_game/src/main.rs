use rand::{thread_rng, Rng};
use std::{cmp::Ordering, io};

fn main() {

    println!("Guess the number game!!");

    let secret: u32 = thread_rng().gen_range(1..=100);
    let mut score: u32 = 0;
    loop {

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        let guess: u32 = match guess.trim().parse()  {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid input");
                continue;
            }
        };

        println!("Your Guess {guess}");

        score=score+1;
        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("Too less!"),
        }
    }
    println!("bhaijaan apka score aya hai {score}");
}
