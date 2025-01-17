use std::io;

fn main() {
    loop {
        let mut input = String::new();

        // Get user input
        println!("Enter your string (or type 'stop' to exit):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        // Trim the input to remove newline characters
        let input = input.trim();

        // Exit condition
        if input.eq_ignore_ascii_case("stop") {
            println!("Goodbye!");
            break;
        }

        // Calculate length and print
        let length = calculate_len(input);
        println!("The length of '{}' is {}", input, length);

        // Create a mutable string from the trimmed input
        let mut mutable_input = input.to_string();
        append(&mut mutable_input);

        // Print the modified string
        println!("Modified String: '{}'", mutable_input);
        println!("Final length: {}", calculate_len(&mutable_input));
    }
}

fn calculate_len(s: &str) -> usize {
    s.len()
}

fn append(s: &mut String) {
    s.push_str(" is great!");
}
