// Program that generates random numbers

use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    let mut trimmed: &str;
    let mut cont = true;
    let mut rng = rand::thread_rng();
    let mut input_text: String;
    let mut n: u16;
    while cont {
        n = rng.gen();
        
        input_text = String::new();

        println!("O numero sorteado foi: {n}");
        print!("Gostaria de continuar? (y/n) ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input_text)
            .expect("Erro na leitura");
        
        trimmed = input_text.trim();
        if vec!["y", "Y"].contains(&trimmed) {
            println!("Sim");
        } else {
            println!("Não");
            cont = false;
        }
    }
}