// 3. Write a program that takes a number as input, and prints whether the number is even or odd.

use std::io;
use std::io::Write;

fn is_even(number: u32) -> bool {
    let reminder = number % 2;
    match reminder {
        0 => return true,
        _ => false
    }
}
fn main() {
    let mut input = String::new();
    
    // My exercise
    print!("Write a number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<u32>() {
        Ok(i) => {
            match is_even(i) {
                true => println!("The number is even"),
                false => println!("The number is odd")
            }
        },
        Err(_) => println!("Leitura inválida!")
    }

    input.clear();

    // ChatGPT snippet
    println!("Enter a number:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let number: i32 = input.trim().parse().unwrap();

    if number % 2 == 0 {
        println!("The number {} is even", number);
    } else {
        println!("The number {} is odd", number);
    }
}