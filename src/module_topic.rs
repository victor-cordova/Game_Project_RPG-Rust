// use std::{io, io::Write};
use std::io::{self, Write};

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        season_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                season_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn module_topic () {
    let mut meal = 
        back_of_house::Breakfast::summer("toast");

    meal.toast = String::from("value");
}