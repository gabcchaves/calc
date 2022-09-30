/* Program to read an parse mathematical expressions from
 * command line. */
use std::io;
use std::collections::HashMap;

// GLOBALS
const OPERATORS: [char; 4] = ['+', '-', '*', '/'];
const SYMBOLS: [char; 2] = ['(', ')'];


fn main() {
    let expr_string = read_string();
    println!("{:?}", convert_to_hashmap(expr_string));
}


// Read string from STDIN.
fn read_string() -> String {
    let mut str_input = String::new();
    io::stdin().read_line(&mut str_input);
    str_input.trim().to_string()
}


// Convert expression string to a hashmap
// that maps operands and operators.
fn convert_to_hashmap(expr_string: String) -> Vec<String> {
    // Create the vector
    let mut expr_vector = Vec::<String>::new();

    // Remove whitespaces
    let temp_expr_string = expr_string.split_whitespace().collect::<String>();

    // Separate operands from operators
    let temp_expr_vector = temp_expr_string.split_inclusive(&SYMBOLS[..]);
    let mut operand = String::new();

    for element in temp_expr_vector {
        for c in element.chars() {
            if c.is_numeric() || c == ',' || c == '.' {
                operand.push(c);
            } else {
                if OPERATORS.contains(&c) || SYMBOLS.contains(&c) { /* ( End of operand.) */
                    expr_vector.push(operand.clone());
                    expr_vector.push(c.to_string());
                    operand = String::new();
                } else {
                    panic!("Invalid expression.");
                }
            }
        }
    }
    expr_vector.push(operand);

    expr_vector
}


// Check if string is an operator
fn is_operator(string: String) -> bool {
    if string.len() == 1 {
        if OPERATORS.contains(&string.chars().last().unwrap()) {
            return true;
        }
    }

    false
}


// Check if string is a symbol
fn is_symbol(string: String) -> bool {
    if string.len() == 1 {
        if SYMBOLS.contains(&string.chars().last().unwrap()) {
            return true;
        }
    }

    false
}
