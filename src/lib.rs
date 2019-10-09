#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

        pub mod back_of_house{
            pub enum Appetizer{
                Soup,
                Salad,
            }

            fn fix_incorrect_order(){
                cook_order();
                super::serve_order();
            }

            fn cook_order(){}

            pub struct Breakfast{
                pub toast: String,
                seasonal_fruit: String,
            }

            impl Breakfast {
                pub fn summer(toast: &str) -> Breakfast{
                    Breakfast {
                        toast: String::from(toast),
                        seasonal_fruit: String::from("Peaches"),
                    }
                }
            }
        }

        pub fn eat_at_restaurant(){
            let order1 = back_of_house::Appetizer::Soup;
            let order2 = back_of_house::Appetizer::Salad;

            let mut meal = back_of_house::Breakfast::summer("Rye");
            meal.toast = String::from("Wheat");
            println!("I'd like {} toast please", meal.toast);
        }
    }
}

pub fn testing_paths(){
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// Define hosting
use crate::front_of_house::hosting;

pub fn test_use_crate(){
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

//// Redefine hosting
// use self::front_of_house::hosting;
//
// pub fn test_use_self(){
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

// Idiomatic use path
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

// Use specific fn from module
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

//// Use different Result from different modules
// use std::fmt;
// use std::io;
//
// fn function1() -> fmt::Result {
//     // --snip--
// }
//
// fn function2() -> io::Result<()> {
//     // --snip--
// }

// Use multiple packages
use std::{cmp::Ordering, io::{self, Write}, collections::*};
