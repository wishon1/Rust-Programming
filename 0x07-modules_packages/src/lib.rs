// 0x07. Rust - Packages, Crates, and Modules

/// Front of house module containing hosting and serving submodules
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Please follow the host to your table");
        }
        
        pub fn seat_at_table() {
            println!("Seated at table")
        }
    }

   pub mod serving {
        pub fn take_order() {
            println!("Taking order")
        }

        pub fn serve_order() {
            println!("Serving order: Pizza")
        }

        pub fn take_payment() {
            println!("Processing payment")
        }
    }
}

/// Function demonstrating the restaurant service flow for a customer
pub fn customer() {
    // using absolute path to call the function
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();
    front_of_house::serving::take_order();
    front_of_house::serving::serve_order();
    front_of_house::serving::take_payment();
}