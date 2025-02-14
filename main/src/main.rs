use std::{collections::HashMap, vec};
use indexmap::IndexSet;
use regex::Regex;

slint::include_modules!();

fn is_valid_identifier(identifier: &str) -> bool {
    let regex: &str = "^[A-Za-z]\\.[a-z$!?_]*$";
    let re = Regex::new(regex).unwrap();

    if re.is_match(identifier)
    {
        return true;
    }
    else
    {
        return false;
    }
}

fn is_valid_type(type_input: &str) -> bool {
    let regex: &str = "![A-Za-z]";
    let re = Regex::new(regex).unwrap();

    if re.is_match(type_input)
    {
        return true;
    }
    else
    {
        return false;
    }
}

fn create_char_set(string: &str) -> IndexSet<char> {
    let mut string_set: IndexSet<char> = IndexSet::new();

    for c in string.chars() {
        string_set.insert(c);
    }

    return string_set;
}

fn main() {
    /*let window: AppWindow = AppWindow::new()?;
    let window_weak = window.as_weak();

    window.on_is_correct(move | user_input | {
        let window = window_weak.unwrap();
        
        let result: String = is_valid_identifier(user_input.to_string());   
        window.set_data_out(result.into());
    });
    
    window.run();
    Ok(())*/

    /*let mut symbol_table: HashMap<char, &str> = HashMap::new();

    let mut tokens: HashMap<&str, Vec<&str>> = HashMap::new();

    tokens.insert("DEL", vec![";"]);
    tokens.insert("OPA", vec!["+", "-", "*", "/"]);*/

    let test_string: &str = "!s s.test;";

    let string_set = create_char_set(test_string);

    for c in string_set.iter() {
        print!("{}", c);
    }
    
}

