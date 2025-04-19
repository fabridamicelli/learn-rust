use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process::exit;

fn main() {
    let secret = rand::rng().random_range(1..=100);

    println!("This is the Guess the number game.\nInput your guess ('quit' or 'exit' to stop the game): ");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess = guess.trim();

        if guess == "quit" || guess == "exit" {
            println!("Goodbye!");
            exit(0)
        };

        println!("Your guess: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("It's a numbers game!. You entered: {guess}");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win! Game over.");
                exit(0)
            }
        }
    }
}
