// 4. Write a program that takes a number as input, and prints the factorial of that number.

use std::io;
use std::io::Write;


fn factorial(number: u32) -> usize {
    if number == 1 { 1 } else  { number as usize * factorial(number - 1) }
}

fn main() {
    let mut input = String::new();
    print!("Write a number: ");
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut input).unwrap();
    let number: u32 = input.trim().parse().unwrap();

    let result = factorial(number);
    println!("Factorial of {}: {}", number, result)
}