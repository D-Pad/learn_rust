

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}


// Using a combination of data types requires the use of multiple 
// generic types in the defintion
#[derive(Debug)]
struct ComboPoint<T, U> {
    x: T,
    y: U
}


#[derive(Debug)]
enum Coordinate<T> {
    Point(Point<T>),
    None
}


impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x 
    }
}


pub fn run() {
   
    println!("\n--- Generics ---");

    let integer = Point { x: 5, y: 7 };
    let floats = Point { x: 1.2, y: 3.14 };
    
    let combo = ComboPoint { x: 2, y: 5.3 };
    let not_combo = Coordinate::Point(Point { x: 12, y: 24 }); 

    println!("Int point: {:?}", integer);
    println!("Float point: {:?}", floats);
    println!("Combo point: {:?}", combo);
    println!("Combo-without-mixed-types point: {:?}", not_combo);
    match not_combo {
        Coordinate::Point(p) => println!("Combo X coordinate: {}", p.x),
        Coordinate::None => println!("No coordinate listed")
    };
}

