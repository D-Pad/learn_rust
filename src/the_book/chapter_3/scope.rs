
fn my_func() {
    let outer_scope: &str = &String::from("OUTER SCOPE");
    let recipient: String;
    let border: &str = &String::from("\n  --------------------");
    {
        recipient = String::from("INNER SCOPE");
        let inner_scope: &str = &recipient;
        println!("\n  TO:   {}{}\n    {}\n    -- {}\n {}", 
            outer_scope, 
            border,
            String::from("Happy birthday"), 
            inner_scope, 
            border
        );
    }
    // 'inner_scope' is no longer accessible from here, but since we 
    // modified 'recipient', we can return a message to the original sender.
    println!("\n  TO:   {}{}\n    {}\n    -- {}\n {}", 
        recipient, 
        border, 
        String::from("Thank you"), 
        outer_scope, 
        border 
    );
}


pub fn run() {
    // Scope demonstration
    println!("\n~~~ SCOPE ~~~");
    my_func();
}


