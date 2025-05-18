

fn arrays() {
    // Arrays in rust are homogenous. All elements of the array must be 
    // of the same type 
    println!("ARRAYS:");
    let num_arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("  Some numbers: {:?}", num_arr);

    // String array
    let fruit: [&str; 3] =  ["Apple", "Banana", "Orange"];
    println!("  Some fruit: {:?}", fruit); 
    println!("  I like {}s, but can't stand {}s", fruit[2], fruit[0]); 
}


fn tuples() {
    
    println!("TUPLES:");
    // A simple tuple declaration
    let biped_1 = ("Alice", 30, false);

    // An annotated tuple declaration
    let biped_2: (String, i32, bool) = ("Zorb".to_string(), 184, true);
  
    println!("  Human: {:?}\n  Alien: {:?}", biped_1, biped_2);
    if biped_2.2 != biped_1.2 {
        println!("  {} was abducted by {}", biped_1.0, biped_2.0);
    } else {
        println!("  {} and {} are good pals.", biped_1.0, biped_2.0);
    }

    // A 2D structure of arrays in a tuple  
    let two_dee: ([&str; 3], [u8; 3]) = (
        ["D-Pad", "Bryan", "Steve"], 
        [13, 21, 34]
    );
    println!("  2D structure: {:?}", two_dee);
}


fn slices() {
    /*
    * Slices are a dynamically sliced view, of a contiguous sequence of 
    * elements. A 'contiguous sequence' refers to a series of elements 
    * placed together in a continuous order without any gaps in between. 
    * It is a pattern used to organize data where elements are stored 
    * consecutively in memory.
    */ 
   
    // Array slices are declared the same way as a string slice (&str)
    println!("SLICES:");
    let my_slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("  Number slice: {:?}", my_slice);

    // String array slices 
    let animals: &[&str] = &["Lion", "Elephant"];
    println!("  &[&str]: {:?}", animals);

    let books: &[&String] = &[
        &"Harry Potter".to_string(), 
        &"Outsiders".to_string()
    ];
    println!("  &[&String]: {:?}", books);

}


fn strings() {
    println!("STRINGS:");
   
    // STRINGS
    // Strings (String) are mutable, growable, owned string types. When they 
    // are declared, they are allocated on the HEAP.
    let mut reinhardt: String = String::from("Hammer"); 
    reinhardt.push_str(" DOWN!");
    println!("  Reinhardt says: {}", reinhardt);
   
    // Declare a string, then reassign it to a new value
    let mut most_played: String = String::from("Mercy");
    println!("  I don't often play {}", most_played);
    most_played = String::from("Reinhardt");
    println!("  My most played hero is {}", most_played);

    // STRING SLICES
    let mut ticker_symbol: String = String::from("ESM");
    let year: u8 = 24;
    ticker_symbol.push_str(&year.to_string());
    let ticker_slice: &str = &ticker_symbol;
    println!("  Ticker symbol: {}", ticker_slice);
    let asset_id: &str = &ticker_slice[0..2];
    println!("  Asset ID: {}", asset_id);
}


pub fn run() {
    /* 
    * Compound types include arrays, tuples, slices, and strings
    */
    println!("\n~~~ COMPOUND_TYPES ~~~");
    arrays();
    tuples();
    slices();
    strings();
}

