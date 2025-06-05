use std::fs::File;
use std::io::{self, Read, Write};



fn read_msg_from_file() -> Result<String, io::Error> {
    let mut msg_file = File::open("hello.txt")?;
    let mut msg = String::new();
    if let Err(e) = msg_file.read_to_string(&mut msg) {
        Err(e)
    } else {
        Ok(msg)
    }
}


fn create_file() {

    let file_contents: Result<String, io::Error> = read_msg_from_file();
    if let Err(e) = file_contents {
        
        // Try to create the 'hello.txt' file... 
        if let Ok(mut file) = File::create("hello.txt") { 
            if let Ok(_) = writeln!(file, "Hello world!") {
                ()
            } else {
                println!("Failed to write data: {}", e);
            }
        } else {
            println!("Failed to create file: {}", e);
        };
    
    } else {
        () 
    }
}

pub fn run() -> String {
    // Try to read the file 
    let mut msg = read_msg_from_file();
    
    // If there's a file error, then it probably doesn't exist. Call the 
    // create function, and read it again.
    if let Err(e) = &msg {
        create_file();
    } else { 
        msg = read_msg_from_file(); 
    }
    msg.expect("Could not read file")
}

