
fn adder(num_1: i32, num_2: i32) -> i32 {
    num_1 + num_2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality_test() {
        assert_eq!(adder(2, 2), 4); 
    }

    #[test]
    fn not_equal_test() {
        assert_ne!(adder(2, 2), 5); 
    }

    #[test] 
    #[should_panic] 
    fn panic_test() {
        assert_eq!(adder(2, 2), 5); 
    }
}


pub fn run() {
    println!("-- Auto Test Modules --");
    println!("Run 'cargo test' to get the intended output of this module")
}

