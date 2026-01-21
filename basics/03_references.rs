// fn main() {
//     let mut s = String::from("hello");

//     let s1 = &s;
//     let s2 = &s;
//     println!("s1:{s1}\ns2:{s2}");
    
//     let s3 = &mut s;
//     println!("{s3}");

// }

// Dangling references
// this block will throw an error: missing lifetime specifier

// fn main() {
//     let some_reference = dangle();
//     println!("{some_reference}");
// }

// fn dangle() -> &String {
//     let some_string = String::from("hello");
//     &some_string
// }