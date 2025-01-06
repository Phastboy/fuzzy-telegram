fn main() {
    println!("hi there!");

    let pi=3.141592;
    println!("{ } value to 3 decimal place is {pi:.*}", "pi", 3);

    let mut temp=1;
    println!("{ } initial value is {temp}", "temp");

    temp=5;
    println!("{ } final value is {temp}", "temp");
}
