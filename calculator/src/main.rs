use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut operator = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut input1).expect("Failed to read");

    println!("Enter an operator (+, -, *, /):");
    io::stdin().read_line(&mut operator).expect("Failed to read");

    println!("Enter second number:");
    io::stdin().read_line(&mut input2).expect("Failed to read");

    // Convert strings to numbers
    let num1: f64 = input1.trim().parse().unwrap();
    let num2: f64 = input2.trim().parse().unwrap();
    let op = operator.trim();

    // Do the math
    let result = if op == "+" {
        num1 + num2
    } else if op == "-" {
        num1 - num2
    } else if op == "*" {
        num1 * num2
    } else if op == "/" {
        num1 / num2
    } else {
        println!("Invalid operator.");
        return;
    };

    println!("Result: {}", result);
    println!("{} {} {} = {}", num1, op, num2, result);
}
