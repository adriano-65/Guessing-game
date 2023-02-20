// To obtain user input and then print the result as output,
// we need to bring the io input/output library into scope.
use std::io;

// We will use rand crate ,a library crate,
// to generate a secret number that the user will try to guess
use rand::Rng;

// The Ordering type is another enum and has the variants Less, Greater, and Equal.
// These are the three outcomes that are possible when you compare two values.
use std::cmp::Ordering;

// run the game
pub fn run() {
    println!("Guess the number!");

    // our secret number
    let secret_number = get_secret_number();

    // counter to track number of attempts
    let mut counter: u32 = 0; // mutable

    loop {
        // get guess
        let guess = get_user_guess();

        // increment counter by one
        counter += 1;

        // convert guess string to integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Compare the guess to the secret number
        // A match expression.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess too small!"),
            Ordering::Greater => println!("Guess too big!"),
            Ordering::Equal => {
                println!("You win! :: But after {counter} attempts");
                break;
            }
        }
    }
}

// Generate a secret number using
fn get_secret_number() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // return expression implicitly. Could also use 'return secret_number'
    secret_number
}

// Let the user input a guess.
fn get_user_guess() -> String {
    // mutable string. Can be able to be modified or changed.
    let mut guess = String::new();

    // prompt user to input a guess
    println!("Please input your guess.");

    // receive user input
    // The & indicates that this argument is a reference,
    // which gives you a way to let multiple parts of your code -
    // access one piece of data without needing to copy that data into memory multiple times.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // Handle possible error

    guess
}

#[cfg(test)]
mod tests {
    // In order to test private functions
    use super::*;

    #[test]
    fn correct_secret_number() {
        // secret_number should be btn 1 and 100
        let secret_number = get_secret_number();
        assert!(secret_number >= 1 && secret_number <= 100);
    }

    #[test]
    #[should_panic]
    fn wrong_secret_number() {
        let secret_number = get_secret_number();
        assert!(secret_number < 1 || secret_number > 100);
    }
}
