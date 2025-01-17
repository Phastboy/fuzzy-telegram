use std::io;

#[derive(Debug)]
struct Rectangle {
    length: f64,
    width: f64,
}

trait Shape {
    fn area(&self)->f64;
    fn perimeter(&self)->f64;
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.length + self.width)
    }
}

fn main() {
    let length = get_input("What is the length of your rectangle?");
    let width = get_input("What is the width of your rectangle?");

    // instance of Rectangle
    let rect = Rectangle { length, width };

    println!("Length: {}", rect.length);
    println!("Width: {}", rect.width);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
}

// Helper function to handle input
fn get_input(prompt: &str) -> f64 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);

        io::stdin()
            .read_line(&mut input)
            .expect("Failed ❌ to read input.");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input ❌. Please enter a valid number."),
        }
    }
}
