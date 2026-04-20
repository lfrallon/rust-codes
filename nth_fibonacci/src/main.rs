use std::io;
use std::io::Write;

fn main() {
    print!("Enter Fibonacci position: ");
    let mut n = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: f64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let fib = fibonacci(n);
    println!("The {}th Fibonacci number is {}", n, fib);
    let fib_binet = fibonacci_binet(n);
    println!("Binet {}th Fibonacci number is {}", n, fib_binet);
}

fn fibonacci(n: f64) -> u64 {
    match n {
        0.0 => 0,
        1.0 => 1,
        _ => fibonacci(n - 1.0) + fibonacci(n - 2.0),
    }
}

fn fibonacci_binet(n: f64) -> u64 {
    // Using Binet's formula to calculate the nth Fibonacci number
    const A: f64 = 1.618034;
    const B: f64 = 1.0 - 1.618034;
    let sqrt_5: f64 = 5.0_f64.sqrt();

    let fib = ((A.powf(n as f64) - B.powf(n as f64)) / sqrt_5).round() as u64;
    fib
}
