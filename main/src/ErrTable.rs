use indexmap::IndexSet;
use std::collections::HashMap;
use std::fs::{OpenOptions, metadata};
use std::io::Write;


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
                
                if !(dtype == "") { // if it has a data type assigned (so a declared, valid identifier, or a constant)
                    line_type_layout_vec.push(dtype.to_string());
                    dtype_lexeme_index_mapping.insert(line_type_layout_vec.len() - 1, dtype_vec_index);
                }

                else { // if its either an operator (AOP) or an equal sign "=" (AS)
                    for (name, token_vec) in tokens.iter(){
                        for token in token_vec {
                            if (inline_lexeme == &token.to_string()) && (*name == "AOP" || *name == "AS") {
                                line_type_layout_vec.push(lexeme.to_string());
                                dtype_lexeme_index_mapping.insert(line_type_layout_vec.len() - 1, dtype_vec_index);
                                break;
                            }
                        }
                    }
                }
                break;
            }
        }      
    }

    return (line_type_layout_vec, dtype_lexeme_index_mapping);
}


// Checks if semantic rules are followed for the current line
pub fn check_semantics(invalid_lexeme_indexes: &mut Vec<u8>, incompatible_dtype: &mut Option<String>, line_dtype: &Vec<String>, dtype_rules: &[DtypeRules; 3]) -> (Vec<u8>, Option<String>) {

    let mut lhs = Vec::new();
    let mut eql = "";
    let mut rhs = Vec::new();

    if !line_dtype.is_empty() {
        if let Some(eq_index) = line_dtype.iter().position(|x| x == "=") {
            lhs = line_dtype[..eq_index].to_vec();      // Left side
            eql = "=";                                   // Equal sign
            rhs = line_dtype[eq_index + 1..].to_vec();  // Right side
        }
    }

    let mut N: usize = 0;

    match lhs[0].as_str() {
        "!s" => { N = 0; },

        "!i" => { N = 1; },

        "!f" => { N = 2; },

        _ => { N = 3; } //if not a valid dtype

    }

    if N != 3 {
        for (index, value) in rhs.iter().enumerate() {
            if (index % 2) != 0 { // Checks if operators (odd indexes) from current line are equal to dtype semantic rules for the N data type
                
                if !(dtype_rules[N].operators.contains(value)) {
                    invalid_lexeme_indexes.push(index as u8 + 2); // +2 to take into account the indexes for the first dtype and the "="
                }
            }
    
            else { // Checks for operands (even indexes)
                if !(dtype_rules[N].permitted_dtypes.contains(value)) {
    
                    if value != "undefined"{
                        invalid_lexeme_indexes.push(index as u8 + 2); // +2 to take into account the indexes for the first dtype and the "="
                    }
                }
            }
        }
    }

    if !(invalid_lexeme_indexes.is_empty()) { //if not empty, means that there was atleast 1 invalid lexeme
        *incompatible_dtype = Some(dtype_rules[N].main_dtype.clone());
    }
    
    return (invalid_lexeme_indexes.clone(), incompatible_dtype.clone())
}

pub fn check_undefined(line_lexemes: &IndexSet<String>, lexemes: &IndexSet<String>, lexeme_types: &Vec<String>, ErrSem_flag: &mut bool) -> (bool, Vec<String>,bool){
    // Check if theres any undefined variable  
    let mut undefined_identifier_flag: bool = false;
    let mut undefined_lexemes: Vec<String> = vec![];        

    for inline_lexeme in line_lexemes {
        for (index_global_lexeme, global_lexeme) in lexemes.iter().enumerate() {
            
            if (inline_lexeme == global_lexeme) && (lexeme_types[index_global_lexeme] == "undefined") {
                *ErrSem_flag = true;
                undefined_lexemes.push(inline_lexeme.to_string());
                undefined_identifier_flag = true;
            }
        }
    }

    return (undefined_identifier_flag, undefined_lexemes, *ErrSem_flag);
}

pub fn write_to_err_table(path: &str, token: &str, lexemas: &Vec<String>, renglon: u16, descripcion: &str) {
    
    // Creates file if it doesnt exist already
    let mut file = OpenOptions::new().create(true).append(true).open(path).unwrap();

    // If file is empty, adds:
    if metadata(path).unwrap().len() == 0 {
        writeln!(file, "{:<10} | {:<35} | {:<8} | {}", "Token", "Lexema", "Renglon", "Descripcion").unwrap();
        writeln!(file, "{:-<10}---{:-<35}---{:-<8}---{:-<30}", "", "", "", "").unwrap();
    }

    // Append the new row
    writeln!(file, "{:<10} | {:<35} | {:<8} | {}", token, format!("{}", lexemas.join(", ")), renglon, descripcion).unwrap();
}