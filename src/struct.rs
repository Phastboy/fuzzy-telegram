#[derive(Debug)]
struct Rectangle {
    length: f64,
    width: f64,
}

impl Rectangle {
    // Method to calculate the area
    fn area(&self) -> f64 {
        self.length * self.width
    }

    // Method to calculate the perimeter
    fn perimeter(&self) -> f64 {
        2.0 * (self.length + self.width)
    }
}

fn main() {
    // Create an instance of Rectangle
    let rect: Rectangle = Rectangle {
        length: 5.0,
        width: 3.0,
    };

    // Access fields and methods
    println!("Length: {}", rect.length);
    println!("Width: {}", rect.width);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
}
