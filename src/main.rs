/* Program to read an parse mathematical expressions from
 * command line. */
use std::io;
use std::collections::HashMap;

fn main() {
    let operators = ['+', '-', '*', '/', '(', ')'];
    println!("> ");
    let mut expr = read_expr(&operators);
}

// Read expression from STDIN
fn read_expr(operators: &[char]) -> Vec<String> { 
    // Read input string
    let mut str_input = String::new();
    io::stdin().read_line(&mut str_input);
    
    // Prepare expression
    let expr = str_input.split_whitespace().collect::<String>();
    let mut expression = Vec::<String>::new();
    let mut operand = String::new();

    for c in expr.chars() {
        if c.is_numeric() {
            operand.push(c);
        } else {
            if operators[..].contains(&c) || c == '.' {
                expression.push(operand.clone());
                expression.push(c.to_string());
                operand = String::new();
            } else {
                panic!("Invalid operator.");
            }
        }
    }
    expression.push(operand.clone());

    expression
}

// Parse expression
fn parse_expr<T>(expr: Vec<String>, operators: &[char]) -> T {
    let mut stack_operands = Vec<T>::new();
    let mut stack_operators = Vec<char>::new();

    for op in expr {
        if operators[..].contains(op.chars()[0]) {
            stack_operators.push(op.chars()[0]);
        } else {
        }
    }
}

// Add two numbers
// fn add<T>(a: T, b: T) -> T {
// }
