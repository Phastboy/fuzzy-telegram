fn find_max(first_num: i64, second_num: i64) -> i64 {
    if first_num>second_num {
        return first_num;
    }
    second_num
}

fn main() {
    let a: i64 = 3;
    let b: i64 = 6;
    let max_num = find_max(a, b);
    //let max_num = if a<b { b } else { a };
    println!("The larger number is: {}", max_num);
}
