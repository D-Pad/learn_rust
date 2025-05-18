pub mod chapter_1;
pub mod chapter_2;
pub mod chapter_3;
pub mod chapter_4;
pub mod chapter_5;
pub mod chapter_6;


pub fn introduction() {
    let div: &str = &String::from("--------------------------------"); 
    println!(
        "{}\nThe Rust Cheat Sheet\nbased on the book\n{}",
        div,
        div
    );
    println!("Check the README files in each\nchapter for detailed info");
}

