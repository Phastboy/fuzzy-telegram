fn i_will_receive_an_arg(arg: i32) {
    println!("you pass {arg} as an argument");
}

fn main() {
    println!("calling a function...");
    i_will_receive_an_arg(4);
    println!("done!");
}
