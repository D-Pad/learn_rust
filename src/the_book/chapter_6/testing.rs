enum TestEnum {
    Number(u8),
    Name(String)
}


pub fn run() {

    // Create a test enum
    let test_enum: TestEnum = TestEnum::Name(String::from("dpad")); 
}

