

pub fn run() {
    println!("\n~~~ SHADOWING ~~~");
    let x: i32 = 25;
    let y: &i32 = &x;
    let x: i32 = 74;

    /* 
    The value that x originally referred to still exists, but the NAME 
    x has been reassigned to a different value. This is called shadowing,
    and causes many people to falsely believe that we're mutating values
    that are supposed to be immutable.
    */
    println!("X: {}\nY: {}\n", x, y);
}


