use std::io;

fn calculate_area(length: f64, width: f64) -> f64 {
    println!("length: {}", length);
    println!("width: {}", width);
    length*width
}

fn main() {
    let mut input = String::new();

    // get length
    println!("Enter the length of the rectangle: ");
    io::stdin().read_line(&mut input).expect("failed ❌ to read input");
    let length: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid ❌ input for length . Please 🙏 enter a valid number.");
            return;
        }
    };

    input.clear();

    // get width
    println!("Enter the width of the rectangle: ");
    io::stdin().read_line(&mut input).expect("failed ❌ to read input");
    let width: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid ❌ input for width. Please 🙏 enter a valid number.");
            return;
        }
    };

    let area: f64 = calculate_area(length, width);
    println!("Area of the rectangle: {:.3}", area); // to 3 d.p
}
