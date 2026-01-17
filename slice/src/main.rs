fn main() {
    let mut s = String::from("hello world!");

    // let hello = &s[..5];
    // let world = &s[6..11];
    // println!("{hello}, {world}")

    let first = first_word(&s[..]);
    println!("{first}");

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
