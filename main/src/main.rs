use regex::Regex;

fn main() {
     
    let re: &str = "[A-Za-z]\\.[a-z$!?_]*";
    println!("{}",re);

    let var = Regex::new(re); //Object class Regex
    let a= var.unwrap(); //unwrap method

    let ejemplo= "s.text";

    if a.is_match(ejemplo)
    {
        println!("correcto ayuuuu");
    }
    else
    {
        println!("NO, INCORRECTO");
    }
}
