
#[derive[Debug]]
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

fn main(){
    let mut count = 0;

    // if-let
    let value = Some(62);
    if let Some(n) = value {
        println!("The value received is: {}", n);
    }

    // if-let-else
    let specific_us_quarter = Some(Coin::Quarter(UsState::Alaska))
    if let Coin::Quarter(state) = specific_us_quarter {
        println!("State quarter from {:?}", state);
    } else { count += 1}
}
