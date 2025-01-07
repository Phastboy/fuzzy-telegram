fn main() {
    println!("checking if a number is an even number...");
    let number: i32=3;
    if number%2==0 {
        println!("The number is an even number");
        return;
    }
    println!("oops! odd number");
}
