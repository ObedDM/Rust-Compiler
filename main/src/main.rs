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

    println!("line sample: \"{}\"", test_string);

    let lexemes: IndexSet<String> = SymTable::generate_lexeme_table(test_string, &tokens);
    print!("lexemes:\n\n{:?}\n\n", lexemes);

    let mut lexeme_types: Vec<String> = SymTable::generate_lexeme_type(lexemes, valid_regex_type_match);

    print!("types:\n\n{:?}\n\n", lexeme_types);

    let mut cat: i8 = -1; //Initializes cat

    match LineCat::categorize_line(test_string) {
        Some(value) =>  {
            cat = value;
        }

        None => {
            println!("\"{}\" cannot be categorized", test_string);
        }
    };

}
