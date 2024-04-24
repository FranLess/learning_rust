// BASIC CONCEPTS

// - Statement: a statement is a piece of code which is no evaluated to an any value
// it means that is doesn't "return" a value to be assigned to a variable

// - Expression: a expression is a piece of code which is evaluated to a value
// it means that it "returns" a value that can be assigned to a variable
fn main() {
    let x = function_with_returned_value(12);
    let y = function_without_returned_value(14);
    println!(
        "Value of x is: {x}
Value of y is: {y}"
    )
}

// if a expression is written to the final of a function will be returned by the function
// without the semicolon at the end
fn function_with_returned_value(x: i32) -> i32 {
    return x + 1;
}
fn function_without_returned_value(x: i32) -> i32 {
    x + 1
}
