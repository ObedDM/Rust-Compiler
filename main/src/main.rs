use std::collections::HashMap;
use indexmap::IndexSet;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

mod SymTable;
mod LineCat;

slint::include_modules!();

fn main() {

    let mut file: File = File::open("code.txt").expect("File not found or cant be opened");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File cannot be read");
    

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

    //Define regex type match
    let valid_regex_type_match: HashMap<&str, Regex> = HashMap::from([
        ("ID", Regex::new("^[A-Za-z]\\.[a-z$!?_]*$").unwrap()),
        ("!s", Regex::new("^\".*\"$").unwrap()),
        ("!i", Regex::new("^[0-9]+$").unwrap()),
        ("!f", Regex::new("^[0-9]+\\.[0-9]+$").unwrap())
    ]);

    //Define tokens
    let mut tokens: HashMap<&str, Vec<char>> = HashMap::new();
    tokens.insert("AS", vec!['=']);
    tokens.insert("DEL", vec![';', ',', ]);
    tokens.insert("SPACE", vec![' ']);
    tokens.insert("AOP", vec!['+', '-', '*', '/', '%']); //Arithmetic operators


    let multiple_lines_test: &str = &contents;   

    let mut lexemes: IndexSet<String> = IndexSet::new();
    let mut lexeme_types: Vec<String> = Vec::new();

    let mut line_counter: u16 = 0;


    //Sample of a line to be processed
    //let test_string: &str = "!s s.test, x.ss, o.a$";

    //let lexemes: IndexSet<String> = SymTable::generate_lexeme_table(test_string, &tokens);

    //let mut lexeme_types: Vec<String> = SymTable::generate_lexeme_type(&lexemes, &valid_regex_type_match);

    for test_string in multiple_lines_test.lines() {

        line_counter += 1;

        println!("{}   {}", line_counter, test_string);

        let line_lexemes = SymTable::generate_lexeme_table(test_string, &tokens);
        let mut line_lexeme_types = SymTable::generate_lexeme_type(&line_lexemes, &valid_regex_type_match);

        let mut cat: i8 = -1; //Initializes cat

        match LineCat::categorize_line(test_string) {
            Some(value) =>  {
                cat = value;
            }

            None => {
                //println!("\"{}\" cannot be categorized", test_string);
            }
        };

        let type_regex = Regex::new("!s|!f|!i").unwrap();

        if (cat == 0) || (cat == 3) || (cat == 4) { // Either dec, dec-aop, or dec-asg. Any type of declaration;
     
            if let Some(substring) = type_regex.find(test_string) { // Finds identifier in the string
                for data_type in line_lexeme_types.iter_mut() {
                    // Update the lexeme type if its "ID"
                    if *data_type == "ID" {
                        *data_type = substring.as_str().to_string();
                    }
                }
            }
        }

        else {
            for data_type in line_lexeme_types.iter_mut() {
                if *data_type == "ID" {
                    *data_type = "".to_string();
                }
            }
        }

        //println!("\n\ncode sample:\n\"{}\"\n\nTable of Symbols:\n", test_string);
        //println!("{:?}\n{:?}", line_lexemes, line_lexeme_types);         

        // Handles duplicated type asignment between lexeme and lexeme_types
        SymTable::handle_duplicate_lexemes(&line_lexemes, &line_lexeme_types, &mut lexemes, &mut lexeme_types);
}
    
    //println!("\n\ncode sample:\n\"{}\"\n\nTable of Symbols:\n", multiple_lines_test);
    println!("\nTable of Symbols:\n\nLexeme: {:?}\nType: {:?}", lexemes, lexeme_types); 

}
