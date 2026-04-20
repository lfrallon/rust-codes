use std::io;
use std::io::Write;

fn main() {
    println!("Temperature Conversion: Fahrenheit to Celsius");

    print!("Please enter the temperature in Fahrenheit: ");
    let mut fahrenheit = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid value.");
            return;
        }
    };
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("{}°F is equal to {:.2}°C.", fahrenheit, celsius);
}
