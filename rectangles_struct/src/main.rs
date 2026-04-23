#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");

    refactoring_tuples();
    debug_macro();
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn refactoring_tuples() {
    let rect1 = (30, 50); // (width, height)

    println!(
        "The area of the rectangle is {} square pixels.",
        refactored_area(rect1)
    );
}

fn refactored_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn debug_macro() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
