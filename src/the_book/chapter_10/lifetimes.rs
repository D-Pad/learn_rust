
struct ImportantExcerpt<'a> {
    part: &'a str,
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}


pub fn run() {
   
    println!("\n--- Lifetimes ---\n");
    
    // Example 1
    let first_name: String = String::from("dpad");
    let last_name: String = String::from("nunyuh");
    let most_chars: &str = longest(&first_name, &last_name);
    println!("Longest name: {most_chars}");
    
    // Example 2
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // Example 3
    let passage: String = String::from("In a galaxy far, far away");
    let some_words: &str = passage.split(',').next().unwrap(); 
    let i = ImportantExcerpt {
        part: some_words
    };
}

