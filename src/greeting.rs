use std::io;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn hello_world() {
    println!("Hello world")
}
#[allow(dead_code)]
pub fn say_hello() {
    let mut name: String = String::new();
    print!("What's your name? ");
    io::stdout().flush().ok();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input.");

    println!("Hello {}", name)
}
