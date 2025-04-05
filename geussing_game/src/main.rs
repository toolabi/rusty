use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{}", secret_number);
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to load message");
    println!("{}", guess);
    let mut numeric_guess: u32 = guess.trim().parse().expect("failed to parse the guess.");
    match numeric_guess.cmp(&secret_number) {
        Ordering::Equal => println!("congarts"),
        Ordering::Greater => println!("guess lower") ,
        Ordering::Less => println!("guess higher") 
    }

}
