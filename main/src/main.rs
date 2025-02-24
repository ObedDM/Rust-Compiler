use std::collections::HashMap;
use std::f32::consts::E;
use std::hash::Hash;
use std::string;
use indexmap::IndexSet;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

mod SymTable;
mod LineCat;
mod ErrTable;

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
        ("!i", Regex::new("^[+-]?[0-9]+$").unwrap()),
        ("!f", Regex::new("^[+-]?[0-9]+\\.[0-9]+$").unwrap())
    ]);

    //Define tokens
    let mut tokens: HashMap<&str, Vec<char>> = HashMap::new();
    tokens.insert("AS", vec!['=']);
    tokens.insert("DEL", vec![';', ',', ]);
    tokens.insert("SPACE", vec![' ']);
    tokens.insert("AOP", vec!['+', '-', '*', '/', '%']); //Arithmetic operators


    //Data type rules (add as needed)
    let string_rules = ErrTable::DtypeRules {
        main_dtype: "!s".to_string(),
        operators: vec!["+".to_string(), "-".to_string()],
        permitted_dtypes: vec!["!s".to_string()] 
    };

    let int_rules = ErrTable::DtypeRules{
        main_dtype: "!i".to_string(),
        operators: vec!["+".to_string(), "-".to_string(), "*".to_string()],
        permitted_dtypes: vec!["!i".to_string()] 
    };

    let float_rules = ErrTable::DtypeRules {
        main_dtype: "!f".to_string(),
        operators: vec!["+".to_string(), "-".to_string(), "*".to_string(), "/".to_string(), "%".to_string()],
        permitted_dtypes: vec!["!f".to_string(), "!i".to_string()] 
    };

    let dtype_rules: [ErrTable::DtypeRules; 3] = [
        string_rules,
        int_rules,
        float_rules
        ];

    let multiple_lines_test: &str = &contents;   

    let mut lexemes: IndexSet<String> = IndexSet::new();
    let mut lexeme_types: Vec<String> = Vec::new();

    let mut line_counter: u16 = 0;

    for test_string in multiple_lines_test.lines() {

        line_counter += 1;

        let empty_line_checker = test_string.replace(" ", "");

        if empty_line_checker.is_empty() {
            println!("line {} is empty\n", line_counter);
            continue;
        }

        let mut cat: i8 = -1; //Initializes cat

        match LineCat::categorize_line(test_string) {
            Some(value) =>  {
                cat = value;
            }

            None => {
                //println!("\"{}\" cannot be categorized", test_string);
            }
        };

        let line_lexemes_vec = SymTable::generate_lexeme_table_as_vec(test_string, &tokens); //Vector containing all lexemes including duplicates (needed for error checking dtype)
        let line_lexemes: IndexSet<String> = line_lexemes_vec.iter().cloned().collect(); //IndexSet that actually tracks the lexemes in a set
        let mut line_lexeme_types = SymTable::generate_lexeme_type(&line_lexemes, &valid_regex_type_match);

        let type_regex = Regex::new("!s|!f|!i").unwrap();

        if (cat == 0) || (cat == 3) || (cat == 4) { // Either dec, dec-aop, or dec-asg. Any type of declaration;
     
            if let Some(substring) = type_regex.find(test_string) { // Finds identifier in the string
                for data_type in line_lexeme_types.iter_mut() {
                    // Update the lexeme type if its "ID"
                    if *data_type == "ID" {
                        *data_type = substring.as_str().to_string();

                        if (cat == 3) || (cat == 4) {
                            break;
                        }
                    }
                }
            }
        }

        for data_type in line_lexeme_types.iter_mut() {
            if *data_type == "ID" { //If theres still any left dtype as ID, becomes undefined
                *data_type = "undefined".to_string();
            }
        }

        // Handles duplicated type asignment between lexeme and lexeme_types
        SymTable::handle_duplicate_lexemes(&line_lexemes, &line_lexeme_types, &mut lexemes, &mut lexeme_types);

        if cat == 3 || cat == 4 || cat == 1 || cat == 2 { // Either aop, asg, dec-aop, dec-asg (not dec since dec represents when a var is declared, thus not having a value assigned)

            let mut invalid_lexeme_indexes: Vec<u8> = vec![];
            let mut incompatible_dtype: Option<String> = None;

            // assigns datatypes to the corresponding lexeme in the current line
            let line_dtype: Vec<String>;
            let lexeme_dtype_mapper: HashMap<usize, usize>;
            
            (line_dtype, lexeme_dtype_mapper) = ErrTable::assign_dtype(&line_lexemes_vec, &lexemes, &lexeme_types, &tokens); 

            println!("line_dtype {:?}", line_dtype);



            (invalid_lexeme_indexes, incompatible_dtype) = ErrTable::check_semantics(&mut invalid_lexeme_indexes, &mut incompatible_dtype, &line_dtype, &dtype_rules);


            println!();
            println!("{:?}", line_dtype);
            
            println!("{:?}", invalid_lexeme_indexes);
            println!("{:?}", incompatible_dtype);
            println!();

            println!("{:?}", line_lexemes);
            
            for (dtypes_index, lexemes_index) in &lexeme_dtype_mapper {
                println!("[{}]{:?} : [{}]{:?}", lexemes_index, line_lexemes_vec[*lexemes_index], dtypes_index, line_dtype[*dtypes_index]);
            }

            println!("{:?}", invalid_lexeme_indexes);

            for dtype_invalid_indexes in invalid_lexeme_indexes.iter().map(|index| *index as usize) {
                let lexeme_real_index = lexeme_dtype_mapper[&dtype_invalid_indexes];

                println!("{}", line_lexemes_vec[lexeme_real_index]);
            }
        }
    }

    //SymTable::write_out_sym_table(lexemes, lexeme_types, "output.txt");

}
