use std::io;

fn main(){
    println!("A Fahrenheit to Celsius Program");

    let mut value = String::new();
    let mut unit = String::new();


    /* read user input */

    // read value
    println!("Enter a value: ");
    io::stdin()
        .read_line(&mut value)
        .expect("Unable to read value");

    // read unit
    println!("Enter the Unit (C) or (F)");
    io::stdin()
        .read_line(&mut unit)
        .expect("Unable to read unit");


    /* Checking and Result */

    // converting to primary types
    let value: f32 = value
                        .trim()
                        .parse()
                        .expect("Unable to convert value to floating points");

    let unit: char = unit
                        .trim()
                        .parse()
                        .expect("Unable to convert value to char, Please enter a character");

    let result = match 
                    unit
                    .to_lowercase()
                    .next()
                    .unwrap()
                {
                    'f' => {
                        let res = fahren_to_celsisu(value);
                        format! ("{}C", res)
                    },
                    'c' => {
                        let res = celsius_to_fahren(value);
                        format! ("{}F", res)
                    },
                    _ => {
                        println!("Please enter (F) or (C) as unit");
                        format! ("Invalid")
                    }
                };

    println!("Your final result is {}", result);
}

fn celsius_to_fahren (value: f32) -> f32 {
    return ((9.0/5.0) * value) + 32.0;
}

fn fahren_to_celsisu (value: f32) -> f32 {
    return (5.0/9.0) * (value - 32.0);
}
