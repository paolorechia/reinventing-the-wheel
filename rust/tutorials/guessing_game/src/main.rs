use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number (1 <= n <= 100)!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is {}:", secret_number);


    loop {
        println!("Please input your guess!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        println!("You've guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Right on!");
                break;
            }
            Ordering::Greater => println!("Too big!")
        }
    }
}