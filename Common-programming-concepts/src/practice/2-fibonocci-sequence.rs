
use std::io;

fn main(){
    let mut counter: i8 = 0;
    let mut first_fibo: i8 = 0;
    let mut next_fibo: i8 = 1;
    let mut last_fibo: i8 = 0;
    let mut nth_number = String::new();

    println!("Enter the nth fibonocci: ");
    io::stdin()
        .read_line(&mut nth_number)
        .expect("Unable to read line");

    let nth_number: i8 = nth_number
                            .trim()
                            .parse()
                            .expect("Unable to parse input");

    println!("{}", first_fibo);
    println!("{}", last_fibo);
    loop {
        if counter >= nth_number { break; }
        last_fibo = first_fibo + next_fibo;
        println!("{}", last_fibo);

        first_fibo = next_fibo;
        next_fibo = last_fibo;
        counter += 1;
    }
}
