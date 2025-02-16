use std::collections::HashMap;
use indexmap::IndexSet;
use regex::Regex;

mod SymTable;
mod LineCat;

slint::include_modules!();

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

    let valid_regex_type_match: HashMap<&str, Regex> = HashMap::from([
        ("ID", Regex::new("^[A-Za-z]\\.[a-z$!?_]*$").unwrap()),
        ("!s", Regex::new("^\".*\"$").unwrap()),
        ("!i", Regex::new("^[0-9]+$").unwrap()),
        ("!f", Regex::new("^[0-9]+\\.[0-9]+$").unwrap())
    ]);

    //Token name
    let mut tokens: HashMap<&str, Vec<char>> = HashMap::new();
    tokens.insert("AS", vec!['=']);
    tokens.insert("DEL", vec![';', ',', ]);
    tokens.insert("SPACE", vec![' ']);
    tokens.insert("AOP", vec!['+', '-', '*', '/', '%']); //Arithmetic operators

    //Sample of a line to be processed
    let test_string: &str = "!s s.test, x.ss, o.a$";

    let lexemes: IndexSet<String> = SymTable::generate_lexeme_table(test_string, &tokens);

    let mut lexeme_types: Vec<String> = SymTable::generate_lexeme_type(&lexemes, &valid_regex_type_match);

    let mut cat: i8 = -1; //Initializes cat

    match LineCat::categorize_line(test_string) {
        Some(value) =>  {
            cat = value;
        }

        None => {
            println!("\"{}\" cannot be categorized", test_string);
        }
    };

    let type_regex: Regex = Regex::new("![A-Za-z]").unwrap();

    if (cat == 0) || (cat == 3) || (cat == 4) { // Either dec, dec-aop or dec-asg. Any type of declaration;
        
        if let Some(substring) = type_regex.find(test_string) { // Finds identifier in the string
            for data_type in lexeme_types.iter_mut() {
                if *data_type == "ID" {
                    *data_type = substring.as_str().to_string();
                }
            }
        }

        else {
            for data_type in lexeme_types.iter_mut() {
                if *data_type == "ID" {
                    *data_type = "".to_string();
                }
            }
        }
    }

    println!("line sample: \"{}\"\n Table of Symbols:\n", test_string);

    println!("{:?}\n{:?}", lexemes, lexeme_types); 

}
