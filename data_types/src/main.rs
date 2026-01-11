fn main() {
    // Scalar types

    // 1. Integer
    let int_var = 42;

    // 2. Floating-point
    let float_var = 3.14;

    println!("Integer: {}, Floating-point: {}", int_var, float_var);

    // Numeric Operations
    let a: i32 = 10;
    let b: i32 = 3;

    let sum = a + b;          // Addition
    let difference = a - b;   // Subtraction
    let product = a * b;      // Multiplication
    let quotient = a / b;     // Division
    let remainder = a % b;    // Modulus
    println!("Sum: {}, Difference: {}, Product: {}, Quotient: {}, Remainder: {}", 
             sum, difference, product, quotient, remainder);

    // Boolean type
    let t = true;
    let f: bool = false;
    println!("Boolean values: t = {}, f = {}", t, f);

    // Character type
    let c = 'z';
    let z: char = 'ℤ';
    println!("Characters: c = {}, z = {}", c, z);

    // Compound types

    // Tuple
    let tup : (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Tuple values: x = {}, y = {}, z = {}", x, y, z);

    // Accessing tuple elements directly
    let first_element = tup.0;
    let second_element = tup.1;
    let third_element = tup.2;

    println!("Accessed directly: first = {}, second = {}, third = {}", first_element, second_element, third_element);

    // Array
    let a : [i32; 5]= [1,2,3,4,5];

    let first = a[0];
    println!("First element of the array: {}", first);


}
