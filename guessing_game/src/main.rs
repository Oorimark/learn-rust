use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Starting guessing game:");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Your secret number is {secret_number}");

    println!("Please input your guess: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to the read line");

    let guess: u32 = guess.trim().parse().expect("Please enter a number!");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater  => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }

    println!("You entered {guess}");
}
