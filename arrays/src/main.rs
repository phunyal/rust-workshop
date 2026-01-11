use std::io;

fn main() {
    let a : [i32; 5] = [10, 20, 30, 40, 50];

    println!("Enter an index to access the array");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Please type a number!");   

    let element = a[index];

    println!("The value at index {} is: {}", index, element);
}
