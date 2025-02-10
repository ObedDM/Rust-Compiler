use regex::Regex;
use std::io;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let window: AppWindow = AppWindow::new()?;
    let window_weak = window.as_weak();

    window.on_is_correct(|| {
        println!("e");
    });
     
    let regex: &str = "[A-Za-z]\\.[a-z$!?_]*";
    let re = Regex::new(regex).unwrap();

    println!("Enter a word: ");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    
    let user_input = user_input.trim();

    if re.is_match(user_input)
    {
        println!("correcto ayuuuu");
    }
    else
    {
        println!("NO, INCORRECTO");
    }

    window.run();
    Ok(())
}

