use std::collections::HashMap;


pub fn run() {
    let players: Vec<&str> = vec!["chad", "brad"];
    let mut scores: HashMap<&str, u8> = HashMap::new();

    for p in &players {
        scores.insert(p, 0); 
    }

    // Add points to a players score
    let count = scores.entry(&players[0]).or_insert(0);
    *count += 1;

    println!("{:?}", scores);
    
}

