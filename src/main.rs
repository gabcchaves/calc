/* Program to read an parse mathematical expressions from
 * command line. */

use std::io;
use std::collections::HashMap;

const OPERATORS_PRECEDENCE: [(&str, u8); 7] = [
    ("+", 1),
    ("-", 1),
    ("*", 2),
    ("/", 2),
    ("(", 3),
    (")", 3),
    ("**", 4),
];
const SYMBOLS: [&str; 7] = ["+", "-", "*", "/", "**", "(", ")"];


// Read string from STDIN
fn read_string() -> String {
    let mut str_input = String::new();
    io::stdin().read_line(&mut str_input);
    str_input.trim().to_string()
}


// Check if the expression string is valid
fn is_valid(expr_string: &String) -> bool {
    // Remove whitespaces
    let temp_expr_string: String = expr_string.split_whitespace().collect();

    // Validate characters
    for c in temp_expr_string.chars() {
        if !c.is_numeric() {
            if !SYMBOLS.contains(&c.to_string().as_str()) {
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
            } else {

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
    }
    
    if open_braces != 0 {
        return false;
    }

    // Check proportion of operands to operators.
    let temp_expr = temp_expr_string.replace("**", "^");
    temp_expr.replace("(", "");
    temp_expr.replace(")", "");

    if !temp_expr.chars().next().unwrap().is_numeric()
        || !temp_expr.chars().last().unwrap().is_numeric() { // Check if starts and ends with operand.
        return false;
    }

    true
}


// Convert expression string to vector.
fn convert_to_vector(expr_string: &String) -> Vec<String> {
    // Remove spaces
    let temp_expr_string = expr_string
        .split_whitespace()
        .into_iter()
        .collect::<String>();

    // Separate operands from operators
    let mut expr_vector = Vec::<String>::new();
    let mut operand = String::new();
    let mut operator = String::new();

    for c in temp_expr_string.chars() {
        if c.is_numeric() {
            if !operator.is_empty() {
                expr_vector.push(operator);
                operator = String::new();
            }
            operand.push(c);
        } else {
            if !operand.is_empty() {
                expr_vector.push(operand);
                operand = String::new();
            }
            if !operator.is_empty() {
                if operator != "*" || c != '*' {
                    expr_vector.push(operator);
                    operator = String::new();
                }
            }
            operator.push(c);
        }
    }
    expr_vector.push(operand);

    expr_vector
}


// Convert expression string to prefix notation.
fn convert_to_prefix_notation(expr_vector: Vec<String>) -> Vec<String> {
    let mut expr: Vec<String> = Vec::new();
    let mut operators_stack: Vec<String> = Vec::new();
    let precedence = HashMap::from(OPERATORS_PRECEDENCE);
    
    for string in expr_vector {
        if SYMBOLS.contains(&string.as_str()) {
            if operators_stack.is_empty()
              || string == "(" 
              || operators_stack.last().unwrap() == "(" 
              || precedence.get(string.as_str()) > precedence.get(operators_stack.last().unwrap().as_str()) {
                operators_stack.push(string);
            } else if string == ")" {
                for operator in &operators_stack {
                    if operator == "(" {
                        break;
                    } else {
                        expr.push(operator.to_string());
                    }
                }
            } else {
                while precedence.get(&operators_stack.last().unwrap().as_str()) >= precedence.get(string.as_str())
                  || operators_stack.last().unwrap() != "("
                  || !operators_stack.is_empty() {
                      expr.push(operators_stack.pop().unwrap());
                      operators_stack.push(string.clone());
                }
            }
        } else {
            expr.push(string);
        }
    }
    expr.push(operators_stack.pop().unwrap());

    expr.into_iter().rev().collect()
}


fn main() {
    let expr_string = read_string();
    if !is_valid(&expr_string) {
        panic!("Expressão inválida.");
    }
    let expr_vector = convert_to_vector(&expr_string);
    println!("{:?}", expr_vector);
    let expr_prefix = convert_to_prefix_notation(expr_vector.clone());
    println!("{:?}", expr_vector);
    println!("{:?}", expr_prefix);
}
