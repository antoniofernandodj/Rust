// 1. Write a program that takes two numbers as input, and prints the sum,
// difference, product, and quotient of the numbers.
use std::io;
use::std::io::Write;

fn main() {
    let mut input_string = String::new();
    let trimmed_a: &str;
    let trimmed_b: &str;
    let mut num_a: u32 = 0;
    let mut num_b: u32 = 0;
    let sum: u32;
    let diff: i32;
    let prod: u64;
    let quot: f32;

    print!("Write a number: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Erro na leitura!");

    trimmed_a = input_string.trim();
    
    match trimmed_a.parse::<u32>() {
        Ok(n) => { num_a = n },
        Err(_) => { println!("Valor inválido!") }
    }

    input_string = String::new();    

    print!("Write the second number: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Erro na leitura!");

    trimmed_b = input_string.trim();

    match trimmed_b.parse::<u32>() {
        Ok(n) => num_b = n,
        Err(..) => println!("valor inválido!")
    }

    sum = num_a + num_b;
    diff = num_a as i32 - num_b as i32;
    prod = num_a as u64 * num_b as u64;
    quot = num_a as f32 / num_b as f32;

    println!(r#"
Results:
The sum between the numbers is {sum},
The difference is {diff},
The prod is {prod},
and the quot is {quot}
"#)

}