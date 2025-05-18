enum IpAddr {
    V4(String),
    V6(String)
}

pub fn run() {
    println!("\n -- Enums and Pattern Matching --");
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}

