use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        print!("Guess the number between 1 and 10 (inclusive):");
        io::stdout().flush().unwrap();
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a number. Try again.");
                continue;
            }
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again."),
            Ordering::Greater => println!("Too big! Try again."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("\nThanks for playing. You can find the source code available at https://github.com/TheRiver/guessing_game");
    
}