// User Struct
#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    email: String
}

impl User {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name); 
    }
}

fn build_new_user(email: String, name: String) -> User {
    let user = User {
        active: true,
        email,
        name 
    };
    user
}

pub fn run() {
    let mut user = User {
        active: true,
        name: String::from("D-Pad"),
        email: String::from("dpad@dpadllc.com")
    };

    user.active = false;
    println!("User name: {}", user.name);
    println!("User active: {}", user.active);
    println!("User email: {}", user.email);

    // Make a new user with the function 
    let new_name: String = String::from("Alecia");
    let new_email: String = String::from("alecia@dpadllc.com");
    
    let new_user: User = build_new_user(new_email, new_name);
    println!("New user: {}", new_user.name);

    // Copy syntax
    let copied_user = User {
        active: false,
        ..user 
    };
    println!("Copied user: {:#?}", copied_user);
    copied_user.greet();

}


