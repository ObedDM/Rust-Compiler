use std::{collections::HashMap};

pub fn categorize_line(line:&str) -> Option<i8> {

    let category_map: HashMap<&str, i8> = HashMap::from([
        ("dec", 0), //line cat = declaring var
        ("aop", 1), //line cat = arithmetic operation
        ("asg", 2),  //line cat = asign into var
        ("dec-aop", 3), //line cat = declaring & arithmetic operation
        ("dec-asg", 4), //line cat = declaring & asign into var
        ]);

    if line.contains('!') { // is it dec?
        if  line.contains('=') { // is it dec only?
            if line.contains(['+', '-', '*', '/', '%']) { // is it dec + aop
                return Some(category_map["dec-aop"]);
            }

            else {
                return Some(category_map["dec-asg"]);
            }
        }
        
        else {
            return Some(category_map["dec"]);
        }   
    }

    else if line.contains(['+', '-', '*', '/', '%']) {
        return Some(category_map["aop"]);
        
        }

    else if line.contains('=') {
        return Some(category_map["asg"]);

    }

    else {
        return None;
    }
}

pub fn uncategorize(category: i8) -> &'static str {
    
    match category {
        0 => "dec",
        1 => "aop",
        2 => "asg",
        3 => "dec-aop",
        4 => "dec-asg",
        _=> "undefined"
    }
}