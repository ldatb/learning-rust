use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() { 
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut number = String::new();

        println!("Guess the number!");

        io::stdin()
        .read_line(&mut number)
        .expect("Failed to read your number");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{number} is not a number");
                continue;
            },
        };

        match number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
        println!("You guessed: {number}\n");
    }
}
