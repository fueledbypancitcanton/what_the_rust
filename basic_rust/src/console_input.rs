use std::io;

pub fn run() {
    println!("Enter a name: ");
    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("failed to readline");
    println!("Hello! {}", name);
}