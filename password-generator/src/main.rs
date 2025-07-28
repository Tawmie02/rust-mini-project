use rand::Rng;
use std::io;

fn main() {
    println!(" Welcome to the Rust Password Generator!");

    // Ask for password length
    println!("How long should the password be?");
    let length = read_number();

    // Ask if letters, numbers, symbols should be included
    println!("Include letters? (y/n)");
    let use_letters = read_yes_no();

    println!("Include numbers? (y/n)");
    let use_numbers = read_yes_no();

    println!("Include symbols? (y/n)");
    let use_symbols = read_yes_no();

    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "!@#$%^&*()-_=+[]{}|;:,.<>?/";

    // Build character set
    let mut charset = String::new();
    if use_letters {
        charset.push_str(letters);
    }
    if use_numbers {
        charset.push_str(numbers);
    }
    if use_symbols {
        charset.push_str(symbols);
    }

    if charset.is_empty() {
        println!("You must include at least one character type!");
        return;
    }

    // Generate password
    let password = generate_password(length, &charset);
    println!("\n Your password: {}", password);
}

// Reads a number from user input
fn read_number() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().parse().expect("Please enter a valid number")
}

// Reads y/n and returns true or false
fn read_yes_no() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

// Generates a random password of given length
fn generate_password(length: usize, charset: &str) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}
