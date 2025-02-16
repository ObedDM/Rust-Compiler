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

pub fn generate_lexeme_type(lexeme_set: IndexSet<String>) -> Vec<String> {
    
    let mut lexeme_type: Vec<String> = vec![];

    let valid_regex_type_match: HashMap<&str, Regex> = HashMap::from([
        ("ID", Regex::new("^[A-Za-z]\\.[a-z$!?_]*$").unwrap()),
        ("!s", Regex::new("^\".*\"$").unwrap()),
        ("!i", Regex::new("^[0-9]+$").unwrap()),
        ("!f", Regex::new("^[0-9]+\\.[0-9]+$").unwrap())
    ]);

    for lexeme in lexeme_set {
        if let Some((data_type, _)) = valid_regex_type_match.iter().find(|(_, re)| re.is_match(&lexeme)) {
            lexeme_type.push(data_type.to_string());
        }

        else {
            lexeme_type.push("".to_string());
        }
    }

    return lexeme_type;
}