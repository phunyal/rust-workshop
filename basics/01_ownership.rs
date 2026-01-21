fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    let y = x;

    println!("x: {}, y: {}", x, y);


    let s1 = String::from("Hello");
    let s2 = s1;

    // println!("{}", s1); // This line would cause a compile-time error
    println!("{}", s2);

    // Making clones
    let s3 = String::from("Hello, world!");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);
}