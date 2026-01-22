#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = build_user(String::from("hello@example.com"), String::from("someusername"));
    println!("{:?}", user1);

    let user2 = User {
        email: String::from("anotherexample@gmail.com"),
        ..user1
    };
    println!("{:?}", user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}