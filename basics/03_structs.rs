#[derive(Debug)]
#[allow(dead_code)]
struct Color(i32, i32, i32);

#[derive(Debug)]
#[allow(dead_code)]

struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let Point(x, y, z) = origin;

    println!("{:#?} {:#?}", black, origin);

    println!("{} {} {}", x, y, z);
}