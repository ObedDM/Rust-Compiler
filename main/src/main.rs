use std::collections::HashMap;
use indexmap::IndexSet;
use regex::Regex;

mod SymTable;
mod linecat;

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

    let lexeme_types: Vec<String> = SymTable::generate_lexeme_type(lexemes);

    print!("types:\n\n{:?}\n\n", lexeme_types);

    match linecat::categorize_line(test_string) {
        Some(cat) =>  {
            let cat_res: &str = &linecat::uncategorize(cat);
            println!("{}", cat_res);
        }

        None => {
            println!("\"{}\" cannot be categorized", test_string);
        }
    };

}
