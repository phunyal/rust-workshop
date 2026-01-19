struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = build_user("someemail@gmail.com".to_string(), "someusername".to_string());

    let user2 = User {
        email: String::from("anotherexample@gmail.com"),
        ..user1
    };

    println!("{} {} {} {}", user2.active, user2.username, user2.email, user2.sign_in_count);
    println!("{} {} {}",user1.email, user1.active, user1.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


}

fn build_user (email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Unit-like Structs

struct AlwaysEqual;
