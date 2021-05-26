use std::io;

use rand::{thread_rng, Rng};

fn main() {
    /// Just for fn
    fn parse_i32(guess: String) -> Option<i32> {
        return guess.trim().parse::<i32>().ok();
    }

    println!("Guess the number!");
    let secret_number = thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match parse_i32(guess) {
            Some(value) => value,
            None => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
