// 5. Write a program that takes a sentence as input,
// and prints the number of vowels and consonants in the sentence.

use std::io;
use std::io::Write;

fn main() {
    let vowels:&str = "aeiou";
    let consonants:&str = "bcdfghjklmnpqrstvwxyz";
    let mut number_vowels: u32 = 0;
    let mut number_consonants: u32 = 0;
    let mut input: String = String::new();

    print!("Enter a phrase: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let phrase: &str = input.trim();

    for c in phrase.chars() {
        if vowels.contains(c) {
            number_vowels += 1
        } else if consonants.contains(c) {
            number_consonants += 1
        }
    }

    println!(r#"
The number of vowels: {number_vowels},
The number of consonants: {number_consonants}
"#)
}