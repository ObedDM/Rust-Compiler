use regex::Regex;
use std::io;

slint::include_modules!();

fn find_out(user_input: String)
{
    let regex: &str = "[A-Za-z]\\.[a-z$!?_]*";
    let re = Regex::new(regex).unwrap();

    println!("{}",user_input);

    if re.is_match(user_input.trim())
    {
        println!("correcto ayuuuu");
    }
    else
    {
        println!("NO, INCORRECTO");
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let window: AppWindow = AppWindow::new()?;
    let window_weak = window.as_weak();

    window.on_is_correct(move | user_input | {
        let window = window_weak.unwrap();
        
        find_out(user_input.to_string());   
        println!("Texto ingresado {}", user_input);
    });
    
    window.run();
    Ok(())
}

