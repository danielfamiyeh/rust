/**
 * Random number guessing game
 */
use rand::Rng;
use std::cmp::Ordering;
use std::io;

/**
 * Parse string input into unsigned 32-bit integer
 */
fn parse_guess(guess: String) -> u32{
    let mut parsed: u32 = 0;

    match guess.trim().parse() {
        Ok(num ) => {parsed = num},
        Err(_) => {println!("Could not parse {guess} into an integer.")},
    }

    return parsed;
}

fn main() {
    println!("Guess what number I'm thinking of!");

    let secret_number = rand::thread_rng().gen_range(0..=100);
    let mut guess: String = "".to_string();

    while parse_guess(guess) != secret_number {
        // Prompt user for input
        guess = String::new();
        println!("Enter a number:");

        // Receive user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // Parse guess into integer so that we can compare
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Compare guess against generated number
        match guess.cmp(&secret_number) {
            Ordering::Equal => println!("Nice one, you got it!"),
            Ordering::Less => println!("Not quite, too low!"),
            Ordering::Greater => println!("Not quite, too high!"),
        }
    }
}
