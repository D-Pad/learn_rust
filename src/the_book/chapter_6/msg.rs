enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


impl Message {
    fn call(&self) {
        // Write code here
    }
}


pub fn run() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
