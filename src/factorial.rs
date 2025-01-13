fn factorial(num: i64) -> i64 {
    if num == 1 || num == 0 {
        return 1;
    }
    num * factorial(num - 1)
}

fn main() {
    let num = 7;
    let result = factorial(num);
    println!("The result of {num} factorial is `{result}`");
}
