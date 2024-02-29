fn main(){
    let mut counter_1: i8 = 0;
    let mut counter_2: i8 = 0;

    // if statement
    let condition: bool = true;
    let y = if condition {3} else {2};

    // loop
    let result = loop {
        counter_1 += 1;
        if counter_1 == 10 {break counter_1;}
    };
    println!("The result is {}", result);

    // label
    'counter_label: loop {
        counter_1 += 1;
        if counter_1 >= 10 {
            loop {
                counter_2 += 2;
                if counter_2 >= 20 {break 'counter_label;}
            }
        }
    }
    println!("Counter 1 is {} and Counter 2 is {}", counter_1, counter_2);
}
