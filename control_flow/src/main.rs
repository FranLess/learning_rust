//NOTE
// all expressions that evaluate () are considered statements, return null
fn main() {
    // if statements are actually expressions so they can be evaluated to variables
    // the only thing to consider here is to be sure every evaluation of if blocks returns
    // the same type
    println!("\n---------------IF EVALUATION");
    let eval = if true { 5 } else { 9 };
    println!("If evaluation is: {eval}");

    let mut counter = 0;
    // loops are aslo expressions, it seems like only "loop" expression can be evaluated
    println!("\n---------------LOOP EVALUATION ");
    let loop_eval = loop {
        if counter == 3 {
            break counter;
        }
        counter += 1;
    };
    println!("Loop evaluation result: {loop_eval}");
    // for expressions are also evaluated but to () which is like a null
    println!("\n---------------FOR IN A RANGE OF 1 TO 6");
    let _for_eval = for i in 1..6 {
        println!("{i}")
    };
    // we can reverse the range of numbers
    println!("\n---------------FOR IN A REVERSE RANGE OF 1 TO 6");
    let _for_eval = for i in (1..6).rev() {
        println!("{i}")
    };

    // we can iterate through an iterator as follows
    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
    // this also evaluates to ()
    println!("\n---------------FOR ITERATING THROUGH AN ARRAY");
    let _for_eval: () = for number in numbers {
        println!("{number}");
    };

    //while loop works as on any other languaje
    let mut counter = 0;
    // this will count until 4
    // and also evaluates ()
    println!("\n---------------WHILE LOOP");
    let _while_eval: () = while counter < 5 {
        println!("Counter is: {counter}");
        counter += 1;
    };
}
