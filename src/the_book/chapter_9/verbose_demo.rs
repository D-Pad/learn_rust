use std::fs::File;
use std::io::{self, Read, Write};


fn read_msg_from_file() -> Result<String, io::Error> {
    let msg_file_result = File::open("hello.txt");

    let mut msg_file = match msg_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut msg = String::new();
    match msg_file.read_to_string(&mut msg) {
        Ok(_) => Ok(msg),
        Err(e) => Err(e),
    }
}


fn create_file() {

    let file_contents: Result<String, io::Error> = read_msg_from_file();
    match file_contents {
       
        // If the file read is successful, then print it
        Ok(msg) => {
            print!("{msg}");
        }
        
        // If the file read fails, the the file probably doesn't exist,
        // and should be created
        Err(e) => {
            
            println!("Failed to read file: {}", e); 
            
            // Try to create the 'hello.txt' file... 
            match File::create("hello.txt") { 
                
                // Write a message to the file. 
                Ok(mut file) => { 
                    match writeln!(file, "Hello world!") {
                        
                        // If writing the data succeeds, then do nothing 
                        Ok(_) => (),
                        
                        // If the data write fails, then notify the user
                        Err(e) => {
                            println!("Failed to write data: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to create file: {}", e);
                }
            };
        }
    };
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

