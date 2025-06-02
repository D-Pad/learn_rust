

pub fn run() {
    let mut people: Vec<String> = Vec::new();
    people.push(String::from("chad"));
    people.push(String::from("brad"));

    println!("{:?}", people);
    println!("winner = {}", people[0]); 
    println!("loser = {}", people[1]); 
}

