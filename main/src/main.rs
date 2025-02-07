use regex::Regex;
use std::io;

fn main() {
     
    let re: &str = "[A-Za-z]\\.[a-z$!?_]*";
    println!("{}",re);

    let var = Regex::new(re); //Object class Regex
    let a= var.unwrap(); //unwrap method

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    println!("{}",entrada);
    let entrada = entrada.trim();

    if a.is_match(entrada)
    {
        println!("correcto ayuuuu");
    }
    else
    {
        println!("NO, INCORRECTO");
    }
}
