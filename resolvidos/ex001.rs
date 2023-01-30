// Simple program that reads numbers from stdin and returns a formatted date

use std::io;
use std::io::Write;

fn main() {
    let mut number1 = 0;
    let mut number2 = 0;
    let mut number3 = 0;
    let mut input_text = String::new();
    print!("Escreva o dia: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Falha na leitura");

    let mut trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => { number1 = i },
        Err(..) => println!("Numero inválido!"),
    };

    input_text = String::new();
    print!("Escreva o mês: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Falha na leitura");
    
    trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => { number2 = i },
        Err(..) => println!("Numero inválido!"),
    };

    input_text = String::new();
    print!("Escreva o ano: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Falha na leitura");

    trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => { number3 = i },
        Err(..) => println!("Erro na leitura"),
    }

    let data = format!("{number1}/{number2}/{number3}");
    println!("A data formatada é: {data}");

}
