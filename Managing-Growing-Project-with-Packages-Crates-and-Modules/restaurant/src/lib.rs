fn deliver_order(){}
mod front_of_house; 

pub use crate::front_of_house::hosting;

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer (toast: &str) -> Self {
            Self {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}


pub fn eat_at_restaurant() {
    // Absolute pwth
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Accessing structs
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Accessing enum -> In enums every field is public if the enum is delcared public
    let order_1 = back_of_house::Appetizer::Soup;
    let order_2 = back_of_house::Appetizer::Salad;
}
