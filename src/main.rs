/* Program to read an parse mathematical expressions from
 * command line. */
use std::io;
// use std::collections::HashMap;


const OPERATORS: [char; 6] = ['+', '-', '*', '/', '(', ')'];


fn main() {
    println!("> ");
    let expr_string = read_string().split(" ").collect::<String>();
    let expr_vector = convert_to_vector(expr_string);
    convert_to_prefix_notation(expr_vector);
}


// Read string from STDIN
fn read_string() -> String {
    let mut str_input = String::new();
    io::stdin().read_line(&mut str_input);
    str_input.trim().to_string()
}


// Convert expression to vector
fn convert_to_vector(expr: String) -> Vec<String> {
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

    expression
}


// Convert expression vector to prefix
fn convert_to_prefix_notation(expr_vector: Vec<String>) -> Vec<String> {
    let mut expression = Vec::<String>::new();
    
    for i in expr_vector {
        if 
    }

    expr_rev
}

// Parse expression
// fn parse_expr(expr: Vec<String>) -> Vec<String> {
// }
