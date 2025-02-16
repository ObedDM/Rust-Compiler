use std::collections::HashMap;
use indexmap::IndexSet;
use regex::Regex;

pub fn is_end_of_lex(c: char, token_map: &HashMap<&str, Vec<char>>) -> (bool, String) {
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

pub fn generate_lexeme_table(line: &str, token_map: &HashMap<&str, Vec<char>>) -> IndexSet<String> {

    let mut lexeme_set: IndexSet<String> = IndexSet::new();
    let mut lexeme: String = String::new();

    //Iterates over the string (line sample)
    for c in line.chars() {
        let (end_of_lexeme, token_type) = is_end_of_lex(c, token_map);

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
    
    return lexeme_set;
}

pub fn generate_lexeme_type(lexeme_set: IndexSet<String>, valid_regex_map: HashMap<&str, Regex>) -> Vec<String> {
    
    let mut lexeme_type: Vec<String> = vec![];

    for lexeme in lexeme_set {
        if let Some((data_type, _)) = valid_regex_map.iter().find(|(_, re)| re.is_match(&lexeme)) {
            lexeme_type.push(data_type.to_string());
        }

        else {
            lexeme_type.push("".to_string());
        }
    }

    return lexeme_type;
}

pub fn create_substring_regex(regex: &Regex) -> Regex {
    let expression = regex.as_str();
    return Regex::new(&format!(".*{}.*", &expression[1..expression.len()-1])).unwrap();
}