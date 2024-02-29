#[derive(Debug)]
enum UsState {
    Alaska,
    Washington,
    Michigan,
}

enum Coin {
    Penny,
    Nickle,
    Dimme,
    Quarter(UsState),
}
fn main() {
    let coin_value: u32 = get_coin_value(Coin::Quarter(UsState::Alaska));
    println!("Your coin value for the coin inserted is : {}", coin_value);

    // matching Options
    let res: Option<i32> = check_if_None(Some(3));
    println!("The result from the check is: {:?}", res);
}

fn get_coin_value(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dimme => 10,
        Coin::Quarter(state) => {
            println!("The US state for the quarter you entered is: {:?}", state);
            let value: u32 = match state {
                UsState::Alaska => 25 + 3,
                UsState::Washington => 25 + 4,
                UsState::Michigan => 25 + 2,
            };
            value
        }
    }
}

fn check_if_None(input: Option<i32>) -> Option<i32> {
    match input {
        Some(i) => Some(i + 1),
        None => None,
    }
}
