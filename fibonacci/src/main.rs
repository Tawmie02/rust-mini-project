use std::io;

fn main() {
    println!("How many Fibonacci numbers should I generate?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let n: u32 = input.trim().parse().expect("Please enter a valid number");

    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        println!("{}", a);
        let next = a + b;
        a = b;
        b = next;
    }
}

