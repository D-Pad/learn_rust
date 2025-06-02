mod restaurant {
    pub mod kitchen {
        pub fn cook_burgers(how_many: u8) {
            println!("Cooking {how_many} burgers");
        } 
    }
}

pub mod gas_station {
   
    pub mod store {
        pub fn buy_gatorade() {
            println!("Buying gatorade");
        }
    }

    pub fn pump_gas(gallons: u8) {
        println!("Pumping {gallons}");
    }
}

fn place_order() { println!("Ordering a burger");}

pub fn run() {
    use restaurant::kitchen; 
    use gas_station::store; 
    store::buy_gatorade(); 
    place_order();
    kitchen::cook_burgers(2); 
}
