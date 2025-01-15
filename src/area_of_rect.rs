fn calculate_area(length: f64, width: f64) -> f64 {
    println!("length: {}", length);
    println!("width: {}", width);
    length*width
}

fn main() {
    let length: f64 = 5.0;
    let width: f64 = 3.5;
    let area: f64 = calculate_area(length, width);
    println!("Area of the rectangle: {:.3}", area); // to 3 d.p
}
