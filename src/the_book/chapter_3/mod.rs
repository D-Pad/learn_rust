mod compound_types;
mod primitive_types;
mod scope;
mod constants;
mod shadowing;
mod loops;


pub fn run() {
    compound_types::run();
    primitive_types::run();
    scope::run();
    constants::run();
    shadowing::run();
    loops::run();
}


