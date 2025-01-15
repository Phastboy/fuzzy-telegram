fn main() {
    let mut language = String::from("Rust");
    let length = calculate_len(&language);
    println!("The length of {} is {}", language, length);
    append(&mut language);
    println!("Modified String: {}", language);
    println!("Final length: {}", calculate_len(&language));
}

fn calculate_len(str: &String) -> usize {
    str.len()
}

fn append(str: &mut String) {
    str.push_str(" is great!");
}
