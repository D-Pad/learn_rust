
trait PersonTitle {
    fn get_title(&self) -> String; 
}


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


struct Doctor {
    name: String
}

impl PersonTitle for Doctor {
    fn get_title(&self) -> String {
        let mut title: String = String::from("Dr. ");
        title.push_str(&self.name);
        title
    }
}


pub fn run() {
    println!("\nTrait demo!");

    let person: Civilian = Civilian { name: String::from("D-Pad") };
    let doc: Doctor = Doctor { name: String::from("Sneed") };

    println!("{}", person.get_title()); 
    println!("{}", doc.get_title()); 
}

