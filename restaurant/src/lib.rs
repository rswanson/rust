mod front_of_house;

pub use crate::front_of_house::hosting;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        // abs path
        // crate::front_of_house::hosting::add_to_waitlist();

        // rel path
        // front_of_house::hosting::add_to_waitlist();

        // using it when brought into scope above
        hosting::add_to_waitlist();

        // Order breakfast in the summer with wheat toast
        let mut meal = crate::back_of_house::Breakfast::summer("wheat");

        // change our toast to the superior rye 
        meal.toast = String::from("Rye");
        println!("I'd like {} toast please", meal.toast);

        let order1 = crate::back_of_house::Appetizer::Soup;
        let order2 = crate::back_of_house::Appetizer::Soup;
    }
}