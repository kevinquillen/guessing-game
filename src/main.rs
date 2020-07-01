use std::io;
use rand::prelude::*;

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
        let number = guess.to_string().trim().parse::<i32>().unwrap();

        if number == answer {
            println!("\nYou got it! {} was the answer!", number);
            break;
        }

        if number < answer {
            println!("\nAlmost! The answer is higher than {}. Guess again!", number);
        } else {
            println!("\nClose! The answer is lower than {}. Guess again!", number);
        }
        
        current_guess = current_guess + 1;        
    }

    println!("\nWhoops! You ran out of guesses. The answer was {}!", answer);
}
