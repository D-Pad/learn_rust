use std::io;
use std::cmp::Ordering;
use rand::Rng;


pub fn run() {
   

    // Initial variables 
    const MAX_GUESS_COUNT: u8 = 8;
    println!("\nRust guessing game!");

    // Generate a random number
    let x: u8 = rand::thread_rng().gen_range(1..=100);

    // Check the answer
    let mut guess_count: u8 = 0;
    loop {        
        
        // Get user input
        println!("\nGuess a number between 1 and 100:");
        let mut guess: String = String::new();
        let _ = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Clean the string input for integer conversion
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => 0, 
        };

        // Check the guess
        if guess > 0 {
            match guess.cmp(&x) {
                Ordering::Less => println!("\nToo small..."),
                Ordering::Greater => println!("\nToo big..."),
                Ordering::Equal => {
                    println!("\nYou got it in {} tries", guess_count + 1);
                    break;
                }
            }
        } else {
            println!("\nInvalid input");
        }

        // Break if the guess count exceeds 3
        guess_count += 1;
        if guess_count == MAX_GUESS_COUNT {
            println!("No more guesses. You lose.");
            println!("The answer was: {x}");
            break;
        }
    }
}


