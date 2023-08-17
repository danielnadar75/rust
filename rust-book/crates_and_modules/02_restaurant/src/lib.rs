pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("pineapple"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // super referes to the immediate parent (unix equivalient is `..` in filesystem)
        super::deliver_order();
    }

    fn cook_order() {}
}

mod customer {

    pub fn eat_at_restaurant() {
        // *** path to modules ***
        // Absolute Path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative Path
        super::front_of_house::hosting::add_to_waitlist();

        // *** Struct with `pub` ***

        // Order a breakfast in the summer with Rye toast.
        let mut meal = super::back_of_house::Breakfast::summer("Honeyoat");
        // Change our mind about what bread we'd like.
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        /*
        Info:   The next line won't compile if we uncomment it; we're not
                allowed to see or modify the seasonal fruit that comes with the meal.
        */
        // meal.seasonal_fruit = String::from("blueberries");

        // *** Enum with `pub` ***
        let order1 = crate::back_of_house::Appetizer::Salad;
        let order2 = super::back_of_house::Appetizer::Soup;

        // *** `use` keyword ***
        // a. non-idiomatic
        use crate::front_of_house::hosting;
        add_to_waitlist();

        // b. idomatic
        use crate::front_of_house::hosting::add_to_waitlist;
        hosting::add_to_waitlist();
    }
}

fn deliver_order() {}

// *** use keyword more examples***

/* - 1. without `as` keyword

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --- snip ---
}

fn function2() -> io::Result<()> {
    // --- snip ---
}
*/

/* - 2. with `as` keyword

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> fmt::Result {
    // --- snip ---
}

fn function2() -> io::IoResult<()> {
    // --- snip ---
}
*/

/* - 3. nested path with use

> Example 01 - {} operator
// a. without nesting
use std::io;
use std::cmp::Ordering;

// b. with nesting
use std::{io, cmp::Ordering}


> Example 02 - self keyworld
// a. wihtout nesting
use std::io;
use std::io::Write

// b. with nesting
use std::io::{self, Write}

 */

/* The Glob Operator - *

  use std::collections::*;

*/
