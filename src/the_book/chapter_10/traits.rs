use std::fmt::{Display, Debug};


trait PersonTitle {
    fn get_title(&self) -> String; 
    
    fn greet(&self) {
        println!("Hello, I am {}.", self.get_title())
    }
}


// CIVILIAN STRUCT
struct Civilian {
    name: String
}

impl PersonTitle for Civilian {
    fn get_title(&self) -> String {
        let mut title: String = String::from("Mr. ");
        title.push_str(&self.name);
        title
    }
}


// DOCTOR STRUCT
struct Doctor {
    name: String
}

impl PersonTitle for Doctor {
    fn get_title(&self) -> String {
        let mut title: String = String::from("Dr. ");
        title.push_str(&self.name);
        title
    }

    fn greet(&self) {
        println!("Greetings! They call me {}.", self.get_title())
    }
}


// SOLDIER STRUCT
struct Soldier {
    name: String
}

impl PersonTitle for Soldier {
    fn get_title(&self) -> String {
        let mut title: String = String::from("Private ");
        title.push_str(&self.name);
        title
    }

    // There is no greet function here, but Soldier can still call it, since
    // we provide a default implementation in the trait definition
}


// Using the 'where' clause
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    42
}



pub fn run() {
    println!("\n--- Traits ---");

    let person: Civilian = Civilian { name: String::from("D-Pad") };
    let doc: Doctor = Doctor { name: String::from("Sneed") };
    let soldier: Soldier = Soldier { name: String::from("Pile") };

    person.greet(); 
    doc.greet(); 
    soldier.greet(); 

    let t = String::from("Testing");
    let u = vec!([1, 2, 3]);
    some_function(&t, &u);
}

