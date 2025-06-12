pub mod generics;
pub mod traits;
pub mod lifetimes;


pub fn run() {
    generics::run();
    traits::run();
    lifetimes::run();
}

