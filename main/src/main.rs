use std::{array, collections::HashMap, string, vec};
use indexmap::IndexSet;
use regex::Regex;

slint::include_modules!();

/*fn is_valid_identifier(identifier: &str) -> bool {
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
}*/

fn create_char_set(string: &str) -> IndexSet<char> {
    let mut string_set: IndexSet<char> = IndexSet::new();

    for c in string.chars() {
        string_set.insert(c);
    }

    return string_set;
}

fn is_end_of_lex(c: char, token_map: &HashMap<&str, Vec<char>>) -> (bool, String) {
    for (token_type, lex_list) in token_map {

        //Checks for possible "end-lexeme" indicating characters; add to the condition as needed
        if (*token_type == "DEL") || (*token_type == "AS") || (*token_type == "SPACE") {
            
            //Checks if current character represents the end of a lexeme
            if lex_list.contains(&c) {
                return (true, token_type.to_string());
            }
        }
    }

    return (false, "".to_string());
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




    /*let test_string: &str = "!s s.test;";

    let string_set = create_char_set(test_string);

    for c in string_set.iter() {
        print!("{}", c);
    }*/

    //Token name
    let mut tokens: HashMap<&str, Vec<char>> = HashMap::new();
    tokens.insert("AS", vec!['=']);
    tokens.insert("DEL", vec![';', ',', ]);
    tokens.insert("SPACE", vec![' ']);

    //Sample of a line to be processed
    let test_string: &str = "!i = i.num + 444;";
 
    let mut lexeme_type: Vec<&str> = vec![];
    let mut lexeme_set: IndexSet<String> = IndexSet::new();

    let mut lexeme: String = String::new();

    //println!("lexemes:\n");

    //Iterates over the string (line sample)
    for c in test_string.chars() {
        let (end_of_lexeme, token_type) = is_end_of_lex(c, &tokens);

        if end_of_lexeme {

            //Checks if lexeme isnt empty to avoid inserting empty string
            if !lexeme.is_empty() {
                lexeme_set.insert(lexeme.clone());
                lexeme.clear();
            }
    
            //Inserts separator as lexeme if its not a space
            if token_type != "SPACE" {
                lexeme.push(c);
                lexeme_set.insert(lexeme.clone());
                lexeme.clear();
            }
        }
        
        else {
            lexeme.push(c); //Constructs the lexeme
        }
    }

    // Inserts any leftover lexeme
    if !lexeme.is_empty() {
        lexeme_set.insert(lexeme);
    }
    
    print!("lexemes:\n\n{:?}\n\n", lexeme_set);


    let valid_regex_type_match: HashMap<&str, Regex> = HashMap::from([
        ("ID", Regex::new("^[A-Za-z]\\.[a-z$!?_]*$").unwrap()),
        ("!s", Regex::new("^\".*\"$").unwrap()),
        ("!i", Regex::new("^[0-9]+$").unwrap()),
        ("!f", Regex::new("^[0-9]+\\.[0-9]+$").unwrap())
    ]);

    for lexeme in lexeme_set {
        if let Some((data_type, _)) = valid_regex_type_match.iter().find(|(_, re)| re.is_match(&lexeme)) {
            lexeme_type.push(*data_type);
        }

        else {
            lexeme_type.push("");
        }
    }

    println!("types:\n\n{:?}\n\n", lexeme_type);

}

