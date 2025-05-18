
fn get_first_word(sentence: &String) -> &str {
    
    // Slice the string 
    for (i, ch) in sentence.bytes().enumerate() {
        if ch == b' ' {
            
            // Using .. is like using : in Python,
            // to get a string slice.
            return &sentence[..i] 
        };
    }
    &sentence[..]
}


pub fn run() {
    
    // Create a string
    let s: String = String::from("Hell no, world!");

    // Slice the string 
    let sl: &str = get_first_word(&s);
    println!("First word: {sl}")
}

