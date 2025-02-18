use std::collections::HashMap;
use indexmap::IndexSet;
use regex::Regex;

pub fn is_end_of_lex(c: char, token_map: &HashMap<&str, Vec<char>>) -> (bool, String) {
    for (token_type, lex_list) in token_map {

        //Checks for possible "end-lexeme" indicating characters; add to the condition as needed
        if (*token_type == "DEL") || (*token_type == "AS") || (*token_type == "SPACE") || (*token_type == "AOP"){
            
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

pub fn generate_lexeme_type(lexeme_set: &IndexSet<String>, valid_regex_map: &HashMap<&str, Regex>) -> Vec<String> {
    
    let mut lexeme_type: Vec<String> = Vec::with_capacity(lexeme_set.len());

    for lexeme in lexeme_set {
        let mut matched_type = String::new(); // Default to ""

        for (data_type, re) in valid_regex_map.iter() {
            if re.is_match(lexeme) {
                matched_type = data_type.to_string();
                break; // Stop at first match
            }
        }

        // If no type found, check if it's an identifier (x.y format)
        if matched_type.is_empty() && lexeme.contains('.') {
            matched_type = "ID".to_string();
        }

        lexeme_type.push(matched_type);
    }

    return lexeme_type;
}

pub fn handle_duplicate_lexemes(line_lexemes: &IndexSet<String>, line_lexeme_types: &Vec<String>, lexemes: &mut IndexSet<String>, lexeme_types: &mut Vec<String>) {
    
    let mut indices_to_remove = Vec::new();
    for (index, lexeme) in line_lexemes.iter().enumerate() {
        if lexemes.contains(lexeme) {
            indices_to_remove.push(index); // Store the indices of duplicates
        }
    }

    for lexeme in line_lexemes {
        lexemes.insert(lexeme.clone());
    }

    let mut filtered_line_lexeme_types: Vec<String> = Vec::new();
    for (index, lexeme_type) in line_lexeme_types.iter().enumerate() {
        if !indices_to_remove.contains(&index) {
            filtered_line_lexeme_types.push(lexeme_type.clone());
        }
    }

    lexeme_types.extend(filtered_line_lexeme_types);
}