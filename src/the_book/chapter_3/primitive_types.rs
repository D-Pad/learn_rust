
fn integers() {
    /*
    * Rust has signed (+ and -) and unsigned (+ only) integer types
    * of different sizes:
    * i8, i16, i32, i64, i128: Signed integers
    * u8, u16, u32, u64, u128: Unsigned integers
    */
    println!("INTEGERS:");
    let x: i32 = -42;
    let y: u8 = 64;
    println!("  Signed value: {}", x);
    println!("  Unsigned value: {}", y);

    let x: i32 = {
        let a: i32 = 12;
        let b: i32 = 10;
        a + b
    };
    println!("  X: {}", x);
}

fn floating() {
    /*
    * Floating point values are declared as f32 and f64
    */
    println!("FLOATS:");
    let small: f32 = 3.14;
    let large: f64 = 1.23424987234;
    println!("  F32: {}", small);
    println!("  F64: {}", large);
}

fn booleans() {
    /*
    * Not much needs to be said here
    */
    println!("BOOLEANS:");
    let is_snowing: bool = false;
    println!("  Snowing: {}", is_snowing);
}

fn characters() {
    /*
    * Character types are quite simple compared to strings in Rust 
    */
    println!("CHARACTERS:");
    let initial: char = 'D'; 
    println!("  My initials start with {}", initial);
}

pub fn run() {
    println!("\n~~~ PRIMITIVE_TYPES ~~~");
    
    // Integers 
    integers();
    floating();
    booleans();
    characters();
}

