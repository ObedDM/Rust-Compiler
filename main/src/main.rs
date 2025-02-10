use regex::Regex;
use std::io;

slint::include_modules!();

fn find_out(user_input: String) -> String {
    let regex: &str = "[A-Za-z]\\.[a-z$!?_]*";
    let re = Regex::new(regex).unwrap();

    if re.is_match(user_input.trim())
    {
        return "Valid input".to_string();
    }
    else
    {
        return "Invalid input".to_string();
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let window: AppWindow = AppWindow::new()?;
    let window_weak = window.as_weak();

    window.on_is_correct(move | user_input | {
        let window = window_weak.unwrap();
        
        let result: String = find_out(user_input.to_string());   
        println!("{}", result.trim());
    });
    
    window.run();
    Ok(())
}

