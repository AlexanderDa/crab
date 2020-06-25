use std::io;
use std::io::prelude::*;
use termion::{color, style};

#[allow(dead_code)]
fn read_values() -> (i32, i32) {
    let mut value1 = String::new();
    let mut value2 = String::new();
    print!("Value 1: ");
    io::stdout().flush().ok();
    io::stdin()
        .read_line(&mut value1)
        .expect("failed to read input.");
    let value1: i32 = value1.trim().parse().expect("invalid input");

    print!("Value 2: ");
    io::stdout().flush().ok();
    io::stdin()
        .read_line(&mut value2)
        .expect("failed to read input.");
    let value2: i32 = value2.trim().parse().expect("invalid input");
    return (value1, value2);
}

/// Perform basic mathematical operations.
/// (+) Addition, (-) Subtraction, (*) Multiplication, (/) Division
/// ## Example
/// ```
/// math::operation("+");
/// ```

#[allow(dead_code)]
pub fn operation(sign: &str) {
    let (value1, value2) = read_values();
    let mut result: i32 = 0;
    if sign == "+" || sign == "-" || sign == "*" || sign == "/" {
        if sign == "+" {
            result = value1 + value2;
        }
        if sign == "-" {
            result = value1 - value2;
        }
        if sign == "*" {
            result = value1 * value2;
        }
        if sign == "/" {
            result = value1 / value2;
        }

        println!(
            "{} {4}{}{5} {} {4}={5} {}",
            value1,
            sign,
            value2,
            result,
            color::Fg(color::LightWhite),
            style::Reset,
        );
    } else {
        println!("Invalid operation")
    }
}

/// Print the multiplication table that the user wants.
/// ## Example
/// ```
/// math::table();
/// ```

#[allow(dead_code)]
pub fn table() {
    let mut value = String::new();
    print!("Value: ");
    io::stdout().flush().ok();
    io::stdin()
        .read_line(&mut value)
        .expect("failed to read input.");
    let value: i32 = value.trim().parse().expect("invalid input");
    for i in 1..11 {
        println!(
            "{} {3}*{4} {} {3}={4} {}",
            value,
            i,
            i * value,
            color::Fg(color::LightWhite),
            style::Reset,
        );
    }
    println!("")
}

/// Print the multiplication tables from 1 to 12
/// ## Example
/// ```
/// math::all_tables();
/// ```

#[allow(dead_code)]
pub fn all_tables() {
    for i in 1..11 {
        for j in 1..11 {
            println!(
                "{} {}*{} {} {}={} {}",
                i,
                color::Fg(color::LightWhite),
                style::Reset,
                j,
                color::Fg(color::LightWhite),
                style::Reset,
                i * j
            );
        }
        println!("")
    }
}
