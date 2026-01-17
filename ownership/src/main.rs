fn main() {
    let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);

    // println!("The length of '{s2}' is {len}");
    let length = calculate_length(&s1);
    println!("The length of is {length}");
    println!("The value in s1 is {s1}"); // s1 can still be used
}

// using tuple approach
// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
// }

// calculate_length function using references approach
fn calculate_length(s: &String) -> usize {
    s.len()
}
