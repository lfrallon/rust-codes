
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

#[derive(Debug)]
enum Option_i32 {
    Some(i32),
    None,
}

#[derive(Debug)]
enum Option_f64 {
    Some(f64),
    None,
}


fn main() {
    let number_list_one = vec![34, 50, 25, 100, 65];

    let mut largest_one = &number_list_one[0];

    for number in &number_list_one {
        if number > largest_one {
            largest_one = number;
        }
    }

    println!("The largest number is {largest_one}");

    let number_list_one = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest_one = &number_list_one[0];

    for number in &number_list_one {
        if number > largest_one {
            largest_one = number;
        }
    }

    println!("The largest number is {largest_one}");

    let number_list_two = vec![34, 50, 25, 100, 65];

    let result_one = largest(&number_list_two);
    println!("The largest number is {result_one}");

    let number_list_two = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result_one = largest(&number_list_two);
    println!("The largest number is {result_one}");

    let number_list_three = vec![34, 50, 25, 100, 65];

    let result_two = largest_i32(&number_list_three);
    println!("The largest number is {result_two}");

    let char_list_one = vec!['y', 'm', 'a', 'q'];

    let result_two = largest_char(&char_list_one);
    println!("The largest char is {result_two}");

    let number_list_four = vec![34, 50, 25, 100, 65];

    let result_three = largest(&number_list_four);
    println!("The largest number is {result_three}");

    let char_list_two = vec!['y', 'm', 'a', 'q'];

    let result_three = largest(&char_list_two);
    println!("The largest char is {result_three}");

    // Point
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("integer {integer:?}; float {float:?}");

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("both_integer {both_integer:?}; both_float {both_float:?}; integer_and_float {integer_and_float:?}");

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let integer_option = Option_i32::Some(5);
    let float_option = Option_f64::Some(5.0);

    println!("integer_option {integer_option:?}; float_option {float_option:?}");
}
