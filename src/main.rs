use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count: u32 = 0;
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                count += 1;
                println!("Too small!");
            }
            Ordering::Greater => {
                count += 1;
                println!("Too big!")
            }
            Ordering::Equal => {
                count += 1;
                println!("You win! It took {count} guesses.");
                break;
            }
        }
    }
}
