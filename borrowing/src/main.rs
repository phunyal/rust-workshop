fn main() {
    let mut s = String::from("hello");

    // Mutable references cannot have more than one reference to that value
    {
        let r1 = &mut s;
    } // r1 goes out of scope here so we can make a new reference
    let r2 = &mut s;

    println!("{r1} and {r2}");
    // change(&mut s);
}

// fn change(some_string: &mut String) {
//     some_string.push_str(", world!");
//     println!("{some_string}")
// }
