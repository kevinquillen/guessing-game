use std::io;
use rand::prelude::*;
use std::cmp::Ordering;
use std::process::exit;

fn main() {
    let max_guess = 10;
    let mut current_guess = 0;
    let answer = rand::thread_rng().gen_range(1, 100);

    println!("\nWelcome to the guessing game!");
    println!("\nPlease enter a number between 1 and 100.");

    while current_guess < max_guess {
        println!("\nGuess #{}. You have {} guesses remaining.", current_guess + 1, (max_guess - current_guess));

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Please enter a number between 1 and 100.");
        let guessed_number = guess.to_string().trim().parse::<i32>().unwrap();

        match guessed_number.cmp(&answer) {
            Ordering::Less => println!("\nAlmost! The answer is higher than {}. Guess again!", guessed_number),
            Ordering::Greater => println!("\nClose! The answer is lower than {}. Guess again!", guessed_number),
            Ordering::Equal => {
                println!("\nYou got it! {} was the answer!", guessed_number);
                exit(0);
            }
        }
        
        current_guess = current_guess + 1;        
    }

    println!("\nWhoops! You ran out of guesses. The answer was {}!", answer);
}
