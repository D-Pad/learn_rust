mod ip_addr;
mod msg;
mod testing;
mod coin_sorting;


pub fn run() {
    ip_addr::run();
    msg::run();
    testing::run();
    coin_sorting::run();
}


