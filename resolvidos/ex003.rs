// Writes a program that takes several numbers from input and
// gives the current sum and the current mean

use::std::io;
use::std::io::Write;

fn main() {
    let mut nums: Vec<f32> = Vec::new();
    let mut trimmed: &str;
    let mut mean: f32;
    let mut sum: f32;
    loop {
        let mut input_string = String::new();
        println!("Write a number: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input_string)
            .expect("Erro na leitura!");

        trimmed = input_string.trim();
        match trimmed.parse::<f32>() {
            Ok(i) => { nums.push(i) },
            Err(_) => println!("Erro na leitura! Tente denovo"),
        }
        println!("Current numbers: {:?}", nums);
        sum = nums.iter().sum();
        println!("The sum is {sum}");
        mean = sum / (nums.len() as f32);
        println!("The current mean is {mean}")
    }
}
