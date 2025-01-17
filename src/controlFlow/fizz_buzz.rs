fn main() {
    let mut number = 0;
    while number < 20 {
        number+=1;
        match (number % 3 == 0, number % 5 == 0) {
            (true, true)=> println!("FizzBuzz"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            (false, false) => println!("{}", number),
        }
    }
}
