use indexmap::IndexSet;
use regex::Regex;
use std::collections::HashMap;

pub fn assign_dtype(line_lexemes: &IndexSet<String>, lexemes: &IndexSet<String>, lexeme_types: &Vec<String>, tokens: &HashMap<&str, Vec<char>>) -> String {
   
    let mut line_type_layout_vec: Vec<&str> = vec![];

        // Categorizes lexemes in current line into their corresponding data types
        for inline_lexeme in line_lexemes {
                
            for (index, lexeme) in lexemes.iter().enumerate() {
                if lexeme == inline_lexeme {
                    let dtype = &lexeme_types[index];
                    
                    if !(dtype == "") {
                        line_type_layout_vec.push(dtype.as_str())
                    }

                    else {
                        for (name, token_vec) in tokens.iter(){
                            for token in token_vec {
                                if (inline_lexeme == &token.to_string()) && (*name == "AOP" || *name == "AS") {
                                    line_type_layout_vec.push(&lexeme);
                                    break;
                                }
                            }
                        }
                    }

                    break;
                }
            }      
        }

        let line_type_layout_string: String = line_type_layout_vec.join("");

        return line_type_layout_string;
}

pub fn check_semantics(rules: &[Regex], s: &str) -> Option<String> {
    
    for re in rules {
        if re.is_match(s) {
            return Some(s.to_string());
        }
    }
    
    return None
}