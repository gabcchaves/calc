/* Program to read an parse mathematical expressions from
 * command line. */
use std::io;
// use std::collections::HashMap;


const OPERATORS: [char; 6] = ['+', '-', '*', '/', '(', ')'];


fn main() {
    println!("> ");
    let expr_string = read_string().split(" ").collect::<String>();
    convert_to_prefix_notation(expr_string);
}


// Read string from STDIN
fn read_string() -> String {
    let mut str_input = String::new();
    io::stdin().read_line(&mut str_input);
    str_input.trim().to_string()
}


// Convert expression to prefix notation
fn convert_to_prefix_notation(expr: String) {
    // Reverse expression string
    let mut expr_rev = expr.chars().rev().collect::<String>();
    expr_rev.replace("(", ")");
    expr_rev.replace(")", "(");

    // Convert to vector
    let mut expression = Vec::<String>::new();
    let mut operand = String::new();
    for c in expr_rev.chars() {
        if c.is_numeric() {
            operand.push(c);
        } else {
            if OPERATORS.contains(&c) {
                expression.push(operand);
                expression.push(c.to_string());
                operand = String::new();
            } else {
                panic!("Expressão inválida.");
            }
        }
    }
    expression.push(operand);

    // Convert to prefix
    let stack_operator = Vec::<char>::new();
}
