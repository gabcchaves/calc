// Program to read an parse mathematical expressions from
// command line.
use std::io;
use std::iter::zip;

fn main() {
    println!("> ");
    // let expr = read_string();
    parse(String::from("1+2"));
}

// Read string from STDIN
fn read_string() -> String {
    let mut str_input = String::new();
    io::stdin().read_line(&mut str_input);
    str_input
}

// Parse expression
fn parse(expr: String) -> f64 {
    let expr_vector = prepare_expr(expr);

    println!("{:?}", expr_vector);

    0.0
}

fn prepare_expr(expr: String) -> Vec<String> {
    let expr: String = expr.split(" ").collect(); // Remove spaces
    let mut operators = Vec::new();
    for c in expr.chars() {
        match c {
            '+' | '-' | '*' | '/' => operators.push(c),
            _ => ()
        }
    }
    let operands = expr.split(&['+', '-', '*', '/'][..]);
    
    let mut expr_vector = Vec::new();
    for i in zip(operands, operators) {
        expr_vector.push(i.0.to_string());
        expr_vector.push(i.1.to_string());
    }

    expr_vector
}
