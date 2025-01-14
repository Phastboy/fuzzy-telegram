fn main() {
    let temp_in_celsius = 25;
    let mut temp_in_fahrenheit = 0.0;
    println!("Initial Temperature in Fahrenheit is: {}", temp_in_fahrenheit);

    temp_in_fahrenheit = (temp_in_celsius as f64 * (9.0 / 5.0)) + 32.0;
    println!("Temperature in Celsius is: {}", temp_in_celsius);
    println!("Temperature in Fahrenheit is: {:.2}", temp_in_fahrenheit);
}
