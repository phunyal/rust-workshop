// fn main() {
//     let s = String::from("Hello");
//     takes_ownership(s);

//     let x = 5;
//     makes_copy(x);
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{s3}");
    println!("{s1}");
}

fn gives_ownership() -> String {
    let some_string = String::from("Hi");
    some_string
}

fn takes_and_gives_back(s: String) -> String {
    s
}