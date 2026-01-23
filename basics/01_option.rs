fn main() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_num: Option<i32> = None;

    println!("{:?}", some_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;

    println!("{sum}");
}