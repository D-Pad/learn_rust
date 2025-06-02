
pub fn run() {
    let mut name: String = String::from("");
    name.push_str("dpad");

    // Add a single character
    name.push('_');
   
    // Add a number to the username
    let num: u8 = 29;
    name.push_str(&format!("{num}")); 

    // Slice a string 
    let original: &str = &name[0..4];

    // Display the user name, the hard way.
    // This demonstrates how to iterate through a string
    for (i, c) in name.chars().enumerate() {
        print!("{c}");
        if i == name.len() - 1 {
            println!();
        }
    }

}



