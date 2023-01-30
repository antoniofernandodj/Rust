// 2. Write a program that takes a string as input, and prints the string in reverse.

use std::io;
use std::io::Write;

fn main() {
    let mut input_string = String::new();
    let mut reversed = String::new();

    // My exercise
    println!("Write a phrase:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_string).unwrap();

    for i in (0..input_string.len()).rev() {
        let c = input_string.chars().nth(i).unwrap().to_string();
        reversed.push_str(&c);
    }

    println!("Reversed: {}\n", reversed);
    input_string.clear();
    reversed.clear();

    // ChatGPT snippet
    println!("Write a phrase:");
    io::stdin().read_line(&mut input_string).unwrap();
    reversed = input_string.chars().rev().collect::<String>();
    
    println!("Reversed: {}\n", reversed);
}