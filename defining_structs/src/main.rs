struct AlwaysEqual;
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// with named lifetime parameter
struct OwnerShipUser<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}

fn main() {
    let _subject = AlwaysEqual;
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin point: ({}, {}, {})", origin.0, origin.1, origin.2);

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("updated@example.com");

    println!("User1 email: {}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("User1: {}, User2: {}", user1.email, user2.email);

    let user3 = build_user(user2.email, user2.username);

    println!("User3 email: {}", user3.email);
    ownership_struct();
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn ownership_struct() {
    let user1 = OwnerShipUser {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };

    println!("User1 email: {}", user1.email);
}
