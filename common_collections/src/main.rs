use std::collections::HashMap;
use std::io::{self, Write};

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
    let s1 = String::new();
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

    hash_maps();

    // List activity
    let lists = vec![1, 5, 3, 9, 2, 4, 6, 2];

    let median_result = get_median(lists.clone());
    let mode_result = get_mode(lists);

    println!("Median: {:#?} Mode: {:#?}", median_result, mode_result);

    let my_string = "Apple Pay";
    let string_result = to_pig_latin(my_string);

    println!("Pig latin: {string_result}");

    company_employees();

}

fn get_median(mut lists: Vec<i32>) -> Option<f64> {
    if lists.is_empty() {
        return None;
    }
    
    // 1. Sort the list (must be mutable)
    lists.sort();

    let len = lists.len();
    let mid = len / 2;

    if len % 2 == 0 {
        // 2. Even length: average of the two middle elements
        Some((lists[mid - 1] + lists[mid]) as f64 / 2.0)
    } else {
        // 3. Odd length: middle element
        Some(lists[mid] as f64)
    }
}

fn get_mode(lists: Vec<i32>) -> Vec<i32> {
    let mut counts = HashMap::new();

    // Step 1: Count occurrences of each number
    for &list in &lists {
        *counts.entry(list).or_insert(0) += 1;
    }

    // Step 2: Filter for numbers with count > 1
    let repeats: Vec<i32> = counts
        .into_iter()
        .filter(|&(_, count)| count > 1)
        .map(|(num, _)| num)
        .collect();

    return repeats;
}

fn to_pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut result = Vec::new();

    for word in text.split_whitespace() {
        let first_char = word.chars().next().unwrap().to_ascii_lowercase();

        if vowels.contains(&first_char) {
            // Rule for vowels
            result.push(format!("{}-hay", word));
        } else {
            // Rule for consonants: split into (first char) and (rest)
            let mut chars = word.chars();
            let first = chars.next().unwrap();
            let rest: String = chars.collect();
            result.push(format!("{}-{}ay", rest, first));
        }
    }

    result.join(" ")
}

fn hash_maps() {
    let mut scores_one = HashMap::new();

    scores_one.insert(String::from("Blue"), 10);
    scores_one.insert(String::from("Yellow"), 50);

    for (key, value) in &scores_one {
        println!("{key}: {value}");
    }

    let team_name = String::from("Blue");
    let score_result = scores_one.get(&team_name).copied().unwrap_or(0);

    println!("Hash map 1: {:#?}", score_result);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map_one = HashMap::new();
    map_one.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    println!("Hash map 2: {:#?}", map_one);

    let mut scores_three = HashMap::new();

    scores_three.insert(String::from("Blue"), 10);
    scores_three.insert(String::from("Blue"), 25);

    println!("Hash map 3: {scores_three:?}");

    let mut scores_four = HashMap::new();
    scores_four.insert(String::from("Blue"), 10);

    scores_four.entry(String::from("Yellow")).or_insert(50);
    scores_four.entry(String::from("Blue")).or_insert(50);

    println!("Hash map 4: {scores_four:?}");

    let text = "hello world wonderful world";

    let mut map_five = HashMap::new();

    for word in text.split_whitespace() {
        let count = map_five.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Hash map 5: {map_five:?}");
}

fn company_employees() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("Commands: 'Add [Name] to [Dept]', 'List [Dept]', 'All', or 'quit'");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "quit" { break; }

        let words: Vec<&str> = input.split_whitespace().collect();

        match words.as_slice() {
            ["Add", name, "to", dept] => {
                company.entry(dept.to_string())
                    .or_insert(Vec::new())
                    .push(name.to_string());
                println!("Added {} to {}.", name, dept);
            }
            ["List", dept] => {
                if let Some(employees) = company.get(*dept) {
                    let mut sorted_employees = employees.clone();
                    sorted_employees.sort();
                    println!("{}: {:?}", dept, sorted_employees);
                } else {
                    println!("Department {} not found.", dept);
                }
            }
            ["All"] => {
                let mut depts: Vec<_> = company.keys().collect();
                depts.sort();
                for dept in depts {
                    let mut names = company[dept].clone();
                    names.sort();
                    println!("{}: {:?}", dept, names);
                }
            }
            _ => println!("Commands: 'Add [Name] to [Dept]', 'List [Dept]', 'All', or 'quit'"),
        }
    }
}