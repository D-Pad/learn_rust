use rand::Rng;


pub fn dice_rolling() {
   
    // Generate a random number
    let roll_value: u8 = rand::thread_rng().gen_range(1..=5); 
   
    // Check the value for a win, or death condition
    match roll_value {
        3 => explode_player(),
        5 => reward_player(),
        other => retry(other) 
    }

    // Arm functions
    fn retry(val: u8) {
        println!("You rolled {val}. Try again!");
    }

    fn explode_player() {
        println!("You're dead!");
    }

    fn reward_player() {
       println!("You win!"); 
    }
}
