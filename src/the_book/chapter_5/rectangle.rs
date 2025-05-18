
// Rectangle struct 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32 
}

impl Rectangle {
    
    fn area(&self) -> u32 {
        self.width * self.height 
    }

    fn rotate(&mut self) {
        let temp_w: u32 = self.width;
        let temp_h: u32 = self.height;
        self.width = temp_h;
        self.height = temp_w;
    }
    
    fn can_fit_inside(&self, other: &Rectangle) -> bool {
        if self.width < other.width && self.height < other.height {
            return true
        }
        false
    }
}

pub fn run() {
    // RECTANGLES 
    let mut rect_a = Rectangle { 
        width: 10,
        height: 4 
    };
    let rect_b = Rectangle { 
        width: 5,
        height: 12
    };
   
    println!("\n  -- Rectangle Structs --");
    println!("The area of the rectangle is: {}", rect_a.area());
    println!("Width and height of 'A': {}x{}", &rect_a.width, &rect_a.height);
    println!("Width and height of 'B': {}x{}", &rect_b.width, &rect_b.height);
    println!("'A' can fit inside of 'B': {}", rect_a.can_fit_inside(&rect_b));
    println!("Rotating rectangle 'A'");
    rect_a.rotate();
    println!("Width and height of 'A': {}x{}", &rect_a.width, &rect_a.height);
    println!("'A' can fit inside of 'B': {}", 
        rect_a.can_fit_inside(&rect_b));
    println!("Width and height of A: {}x{}", &rect_a.width, &rect_a.height);
    
    Rectangle::rotate(&mut rect_a);
    println!("Rotating rectangle 'A'");
    println!("Width and height of 'A': {}x{}", &rect_a.width, &rect_a.height);
}

