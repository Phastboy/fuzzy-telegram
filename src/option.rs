fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator==0.0 {
        return None;
    }
    // return Some(numerator/denominator);
    Some(numerator/denominator)
}

fn main() {
    let numerator = 7.4;
    let denominator = 0.0;
    let result=divide(numerator, denominator).unwrap_or_else(|| {
        println!("Division by zero is undefined.");
        0.0
    });
    println!("{} divided by {} is {}", numerator, denominator, result);
}
