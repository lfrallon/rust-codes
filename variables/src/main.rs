use std::io;

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("Spaces length is: {spaces}");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess is: {guess}");

    // addition
    let sum = 5 + 10;
    println!("Sum of 5 + 10 is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The difference of 95.5 - 4.3 is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The product of 4 * 30 is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!(
        "The quotient of 56.7 / 32.2 is: {quotient} & a Truncated value of -5 / 3 is: {truncated}"
    );

    // remainder
    let remainder = 43 % 5;
    println!("Remainder of 43 % 5 is: {remainder}");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x, y and z: {x}, {y}, {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("From tuples: {five_hundred}, {six_point_four} and {one}");

    // array
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // array of String literals
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    // TODO: Catch index out of bounds
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
