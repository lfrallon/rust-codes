#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Vectors
    let v1: Vec<i32> = Vec::new();
    println!("Vector 1: {:#?}", v1);

    let v2 = vec![1, 2, 3];
    println!("Vector 2: {:#?}", v2);

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    println!("Vector 3: {:#?}", v3);

    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v4.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{i}");
    }

    // mutable
    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        *i += 50;
    }
    println!("Vector 6: {:#?}", v6);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("Vector 7: {:#?}", row);

    // String
    let mut s1 = String::new();
    println!("String 1: {}", s1);

    let data = "initial contents";

    let s2 = data.to_string();
    println!("String 2: {}", s2);

    // The method also works on a literal directly:
    let s3 = "initial contents".to_string();
    println!("String 3: {}", s3);

    let s4 = String::from("initial contents");
    println!("String 4: {}", s4);

    let mut s5 = String::from("foo");
    s5.push_str("bar");
    println!("String 5: {}", s5);

    let mut s6 = String::from("foo");
    let s7 = "bar";
    s6.push_str(s7);
    println!("String 6: {s6}");
    println!("String 7: {s7}");

    let mut s8 = String::from("lo");
    s8.push('l');
    println!("String 8: {s8}");

    let s9 = String::from("Hello, ");
    let s10 = String::from("world!");
    let s11 = s9 + &s10; // note s9 has been moved here and can no longer be used
    println!("String 11: {s11}");

    let s12 = String::from("tic");
    let s13 = String::from("tac");
    let s14 = String::from("toe");

    let s15 = s12 + "-" + &s13 + "-" + &s14;
    println!("String 15: {s15}");

    let s16 = String::from("tic");
    let s17 = String::from("tac");
    let s18 = String::from("toe");

    let s19 = format!("{s16}-{s17}-{s18}");
    println!("String 19: {s19}");

    let hello = "Здравствуйте";
    let answer = &hello[0..4];
    println!("Answer: {}", answer);

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
