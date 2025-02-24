use indexmap::IndexSet;
use regex::Regex;
use std::collections::HashMap;

pub struct DtypeRules { // eg: ["!f", ["+", "-", "*", "/", "%"], ["!f", "!i"]] ----> !f=!i+!f*!i 
    pub main_dtype: String,
    pub operators: Vec<String>,
    pub permitted_dtypes: Vec<String>
}

pub fn assign_dtype(line_lexemes_vec: &Vec<String>, lexemes: &IndexSet<String>, lexeme_types: &Vec<String>, tokens: &HashMap<&str, Vec<char>>) -> (Vec<String>, HashMap<usize, usize>) {

    let mut dtype_lexeme_index_mapping: HashMap<usize, usize> = HashMap::new(); //key: line_type_layout index (that same lexeme but its index as as stored data type in line_type_layout)
                                                                       //value: index from lexeme_vec (basically the whole line's lexeme that admits repeated values)
    let mut line_type_layout_vec: Vec<String> = vec![];

    // Categorizes lexemes in current line into their corresponding data types
    for (dtype_vec_index, inline_lexeme) in line_lexemes_vec.iter().enumerate() {
                
        for (global_lexemes_index, lexeme) in lexemes.iter().enumerate() {
            if lexeme == inline_lexeme {
                let dtype = &lexeme_types[global_lexemes_index];
                
                if !(dtype == "") { // if it has a data type assigned (so a declared, valid identifier)
                    line_type_layout_vec.push(dtype.to_string());
                }

                else { // if its either an operator (AOP) or an equal sign "=" (AS)
                    for (name, token_vec) in tokens.iter(){
                        for token in token_vec {
                            if (inline_lexeme == &token.to_string()) && (*name == "AOP" || *name == "AS") {
                                line_type_layout_vec.push(lexeme.to_string());
                                break;
                            }
                        }
                    }
                }

                if !(line_type_layout_vec.is_empty()) {
                    dtype_lexeme_index_mapping.insert((line_type_layout_vec.len() - 1), dtype_vec_index);
                }

                break;
            }
        }      
    }
    
   
    for (dtypes_index, lexemes_index) in &dtype_lexeme_index_mapping {
        println!("[{}]{:?} : [{}]{:?}", lexemes_index, line_lexemes_vec[*lexemes_index], dtypes_index, line_type_layout_vec[*dtypes_index]);
    }

    return (line_type_layout_vec, dtype_lexeme_index_mapping);
}

pub fn check_semantics(invalid_lexeme_indexes: &mut Vec<u8>, incompatible_dtype: &mut Option<String>, line_dtype: &Vec<String>, dtype_rules: &[DtypeRules; 3]) -> (Vec<u8>, Option<String>) {

    let mut lhs = Vec::new();
    let mut eql = "";
    let mut rhs = Vec::new();

    // checks if semantic rules are followed for the current line
    if !line_dtype.is_empty() {
        if let Some(eq_index) = line_dtype.iter().position(|x| x == "=") {
            lhs = line_dtype[..eq_index].to_vec();      // Left side
            eql = "=";                                   // Equal sign
            rhs = line_dtype[eq_index + 1..].to_vec();  // Right side
        }
    }

    match lhs[0].as_str() {
        "!s" => {
            for (index, value) in rhs.iter().enumerate() {
                if (index % 2) != 0 { // Checks if operators (odd indexes) from current line are equal to string semantic rules
                    
                    if !(dtype_rules[0].operators.contains(value)) {
                        invalid_lexeme_indexes.push(index as u8 + 2); // +2 to take into account the indexes for the first dtype and the "="
                    }
                }

                else { // Checks for operands (even indexes)
                    if !(dtype_rules[0].permitted_dtypes.contains(value)) {
                        invalid_lexeme_indexes.push(index as u8 + 2); // +2 to take into account the indexes for the first dtype and the "="
                    }
                }
            }

            if !(invalid_lexeme_indexes.is_empty()) { //if not empty, means that there was atleast 1 invalid lexeme
                *incompatible_dtype = Some("!s".to_string());
            }
        },

        "!i" => {
            for (index, value) in rhs.iter().enumerate() {
                if (index % 2) != 0 { // Checks if operators (odd indexes) from current line are equal to integer semantic rules
                    
                    if !(dtype_rules[1].operators.contains(value)) {
                        invalid_lexeme_indexes.push(index as u8 + 2); // +2 to take into account the indexes for the first dtype and the "="
                    }
                }

                else { // Checks for operands (even indexes)
                    if !(dtype_rules[1].permitted_dtypes.contains(value)) {
                        invalid_lexeme_indexes.push(index as u8 + 2); // +2 to take into account the indexes for the first dtype and the "="
                    }
                }
            }

            if !(invalid_lexeme_indexes.is_empty()) { //if not empty, means that there was atleast 1 invalid lexeme
                *incompatible_dtype = Some("!i".to_string());
            }
        },

        "!f" => {
            for (index, value) in rhs.iter().enumerate() {
                if (index % 2) != 0 { // Checks if operators (odd indexes) from current line are equal to float semantic rules
                    
                    if !(dtype_rules[2].operators.contains(value)) {
                        invalid_lexeme_indexes.push(index as u8 + 2); // +2 to take into account the indexes for the first dtype and the "="
                    }
                }

                else { // Checks for operands (even indexes)
                    if !(dtype_rules[2].permitted_dtypes.contains(value)) {
                        invalid_lexeme_indexes.push(index as u8 + 2); // +2 to take into account the indexes for the first dtype and the "="
                    }
                }
            }

            if !(invalid_lexeme_indexes.is_empty()) { //if not empty, means that there was atleast 1 invalid lexeme
                *incompatible_dtype = Some("!f".to_string());
            }
        }
        _ => {
            *incompatible_dtype = Some("invalid data type".to_string());
        }
    }
    
    return (invalid_lexeme_indexes.clone(), incompatible_dtype.clone())
}