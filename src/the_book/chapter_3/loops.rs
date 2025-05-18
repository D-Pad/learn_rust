

fn loop_sample() {

    // Using a 'loop' is a good way to loop for some arbitrary number
    // of times, until 'break' or 'return' is reached.
    let mut count: u8 = 0; 
    let total: u8 = loop {
        count += 1;
        if count == 25 {
            break count;
        }
    };
    println!("Total: {total}");
}

fn for_sample() {
    let arr: [u8; 5] = [10, 20, 30, 40, 50];
    for num in arr {
        println!("Number: {num}");
    }
}

fn while_sample() {
    let mut count: u8 = 0;
    while count < 255 {
        count += 1;
    }
    println!("Count: {count}");
}

pub fn run() {
    loop_sample();
    for_sample();
    while_sample();
}

