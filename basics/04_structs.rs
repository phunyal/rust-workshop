// area using separate variables in a function
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!("The area of the rectangle is {} square pixels.", area(width1, height1));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// area using tuple type
// fn main() {
//     let rectangle = (30, 50);
//     println!("The area of the rectangle is {} square pixels.", area(rectangle));

// }

// fn area(rect: (u32, u32)) -> u32 {
//     rect.0 * rect.1
// }

// using structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area(&rectangle));
    // println!("Rectangle Struct: {:?}", rectangle);
    dbg!(&rectangle);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}