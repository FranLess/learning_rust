fn main() {
    //variable shadowing
    let mut x = 5i8;
    //Defautl type of an infered integer is i32
    let integer = 4;
    //default type of a decimal number, "1.5" is f64 which has a double precision
    // it is cause in most of modern computers it is almost as fast as f32
    let decimal_number = 1.4;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // TUPLES
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // unpacking tuples
    let (x, y, z) = tup;
    println!("VALUES BY UNPACKING OR DESTRUCTURING");
    println!(
        "The value of y is: {y}.
The value of x is: {x}.
The value of z is: {z}"
    );
    // we can access to tuple positions with the tuple name followed by .index
    let second_tuple = tup;
    let value_one = second_tuple.0;
    let value_two = second_tuple.1;
    let value_three = second_tuple.2;
    println!("VALUES BY INDEX");
    println!(
        "The value of value one is: {value_one}.
The value of value two is: {value_two}.
The value of value three is: {value_three}"
    )
}
