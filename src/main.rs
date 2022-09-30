/* Program to read an parse mathematical expressions from
 * command line. */

use std::io;
use std::collections::HashMap;

const SYMBOLS: [(&str, u8); 7] = [
    ("+", 1),
    ("-", 1),
    ("*", 2),
    ("/", 2),
    ("(", 3),
    (")", 3),
    ("**", 4),
];

const OPERATORS_CHARS: [char; 4] = ['+', '-', '*', '/'];


// Read string from STDIN
fn read_string() -> String {
    let mut str_input = String::new();
    io::stdin().read_line(&mut str_input);
    str_input.trim().to_string()
}


// Check if the expression string is valid
fn is_valid(expr_string: String) -> bool {
    // Remove whitespaces
    let temp_expr_string: String = expr_string.split_whitespace().collect();

    // Validate characters
    for c in temp_expr_string.chars() {
        if !c.is_numeric() {
            if !OPERATORS_CHARS.contains(&c) {
                return false;
            }
        }
    }

    // Check expression syntax
    let mut open_braces = 0;
    let mut operator_acc = 0;               // For counting consecutive operators (++, +++, --, etc).
    let mut last_operator: char = ' ';      // To compare with the current operator.
    for c in temp_expr_string.chars() {
        if c == '(' {
            open_braces += 1;
        } else {
            if c == ')' {
                open_braces -= 1;
            }

            // Check double operator
            if !c.is_numeric() {
                operator_acc += 1;

                if operator_acc >= 2 {
                    if c != last_operator {
                        return false;
                    } else {
                        if c == '*' && operator_acc > 2 {
                            return false;
                        }

                        if c != '*' {
                            return false;
                        }
                    }
                }
                last_operator = c;
            } else {
                operator_acc = 0;
            }
        }
    }
    
    if open_braces != 0 {
        return false;
    }

    // Check proportion of operands to operators.
    let mut temp_expr = temp_expr_string.replace("**", "^");
    temp_expr.replace("(", "");
    temp_expr.replace(")", "");

    if !temp_expr.chars().next().unwrap().is_numeric()
        || !temp_expr.chars().last().unwrap().is_numeric() { // Check if starts and ends with operand.
        return false;
    }

    true
}


fn main() {
    let expr_string = read_string();
    println!("{:?}", is_valid(expr_string));
}
