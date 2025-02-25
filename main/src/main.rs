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

fn main() -> Result<(), Box<dyn std::error::Error>>{  

    let window: AppWindow = AppWindow::new()?;
    let window_weak = window.as_weak();

    window.on_is_correct(move | user_input | {
        let window = window_weak.unwrap();
        
        let result: String = is_valid_identifier(user_input.as_str()).to_string();   
        window.set_data_out(result.into());
    });

    window.on_probar_codigo(move |code_input| {

    let mut contents = code_input.to_string();

    let mut file = std::fs::OpenOptions::new().create(true).write(true).truncate(true).open("code.txt").unwrap();
    writeln!(file, "{}", contents).unwrap();



    // Empieza la logica del programa
    let mut file: File = File::open("code.txt").expect("File not found or cant be opened");

    
    // Creates new ErrTable file (overwrites if it exists)
    File::create("ErrTable.txt").expect("Unable to clear file");

    //Define regex type to match
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

    let mut ErrSem_counter: u16 = 0;
    let mut ErrSem_flag: bool = false; 

    for test_string in multiple_lines_test.lines() {

        line_counter += 1;

        let empty_line_checker = test_string.replace(" ", "");

        if empty_line_checker.is_empty() {
            //println!("Renglon: {} esta vacio", line_counter);
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

        let mut undefined_identifier_flag: bool = false;
        let mut undefined_lexemes: Vec<String> = vec![];    

        (undefined_identifier_flag, undefined_lexemes, ErrSem_flag) = ErrTable::check_undefined(&line_lexemes, &lexemes, &lexeme_types, &mut ErrSem_flag);
        
        if undefined_identifier_flag && ErrSem_flag { // If theres an undefined lexeme, passes onto next line and creates register into Error Table (above)
            ErrSem_counter += 1;
            ErrTable::write_to_err_table("ErrTable.txt", &format!("Err{}", ErrSem_counter), &undefined_lexemes, line_counter, "variable indefinida");
            //println!("Token: Err{}, Renglon: {}, Lexemas: {:?}, Descripcion: Variable indefinida", ErrSem_counter, line_counter, undefined_lexemes);
            //continue;
        }


        let mut invalid_lexeme_indexes: Vec<u8> = vec![];
        let mut incompatible_dtype: Option<String> = None;

        let mut invalid_lexemes: Vec<String> = vec![];

        if cat == 3 || cat == 4 || cat == 1 || cat == 2 { // Either aop, asg, dec-aop, dec-asg (not dec since dec represents when a var is declared, thus not having a value assigned)

            // assigns datatypes to the corresponding lexeme in the current line
            let line_dtype: Vec<String>;
            let lexeme_dtype_mapper: HashMap<usize, usize>;
            
            (line_dtype, lexeme_dtype_mapper) = ErrTable::assign_dtype(&line_lexemes_vec, &lexemes, &lexeme_types, &tokens); 
            
            (invalid_lexeme_indexes, incompatible_dtype) = ErrTable::check_semantics(&mut invalid_lexeme_indexes, &mut incompatible_dtype, &line_dtype, &dtype_rules);
            
            //println!("vec: {:?}", line_lexemes_vec);
            //println!("dtype: {:?}", line_dtype);

            /* Only checks if assign_dtype is mapping correctly indexes   
            for (dtypes_index, lexemes_index) in &lexeme_dtype_mapper {
                println!("[{}]{:?} : [{}]{:?}", lexemes_index, line_lexemes_vec[*lexemes_index], dtypes_index, line_dtype[*dtypes_index]);
            }*/

            for dtype_invalid_indexes in invalid_lexeme_indexes.iter().map(|index| *index as usize) {
                let lexeme_real_index = lexeme_dtype_mapper[&dtype_invalid_indexes];
                invalid_lexemes.push(line_lexemes_vec[lexeme_real_index].to_string());
            }
        }

        match incompatible_dtype {
            Some(invalid_type) => {
                ErrSem_counter += 1;
                ErrTable::write_to_err_table("ErrTable.txt", &format!("Err{}", ErrSem_counter), &invalid_lexemes, line_counter, &format!("Incompatibilidad de tipos, {}", invalid_type));
                //println!("Token: Err{}, Renglon: {}, Lexemas: {:?}, Descripcion: Incompatibilidad de tipos: {}", ErrSem_counter, line_counter, invalid_lexemes, invalid_type);
            }
            None => {
                //println!("Renglon: {}, no tiene ningun lexema invalido", line_counter);
            }
        }
    }

    SymTable::write_out_sym_table(lexemes, lexeme_types, "SymTable.txt");

    });
    
    window.run();

    Ok(())
}
