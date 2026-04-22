fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    mutable_reference();
    dangling_references();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

fn mutable_reference() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangling_references() {
    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
    println!("reference_to_nothing: {reference_to_nothing}");
}

// fn dangle() -> &String {
//     // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope and is dropped, so its memory goes away.
//   // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
