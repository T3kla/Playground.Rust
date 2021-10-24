use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rnd = rand::thread_rng().gen_range(1..101);

    println!();
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::with_capacity(10);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!();
        print!("You guessed {}", guess);

        match guess.cmp(&rnd) {
            Ordering::Less => println!(" but it's too small!"),
            Ordering::Greater => println!(" but it's too big!"),
            Ordering::Equal => {
                println!(" and you are right, you win!");
                break;
            }
        }
    }
}
