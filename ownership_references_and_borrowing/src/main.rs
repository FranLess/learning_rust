fn main() {
    //ownership is the way memory storage of complex data type
    // stores that data so it doesn't overload our memory

    // complex data is prefered to not have multiple alocations
    // in heap, so the way rust keep reference of these values/data
    // is moving them around and deleting/freeing the data after
    // the scope of it ends

    //SCOPE OF A VARIABLE ENDING
    {
        let string = String::from("Hola mundo.");
        let x = 9;
        println!("{string} the value of x is: {x}");
    } // after this scope "string" and "x" are not longer available
      // println!("{string} the value of x is: {x}"); //this code should fail

    // MOVING A VARIABLE VALUE TO ANOTHER VARIABLE
    let complex_data_type = String::from("dynamic string");
    let complex_data_type_2 = complex_data_type;
    //we can create a full copy, not to move a value as follows
    let complex_data_type_3 = complex_data_type_2.clone();
    //after this "complex_data_type" variable is not longer available
    // println!("value of complex data type is : {complex_data_type}"); // this code should fail
    println!("The value of complex data type is moved to: {complex_data_type_2}");
    println!("The value of complex data type is cloned to: {complex_data_type_3}");

    // The moves also expected to hapen inside a function so we have the following
    let first_value = String::from("Moving a value");
    //we can retrieve the value from the function so we can use it
    let second_value = borrowing_variable(first_value);
    // if we try to use "fist_value" here we'll get an error
    // print!("{first_value}");// this code sould fail
    println!("The fist value was moved to second value: {second_value}");

    // but we can pass complex data by reference so the value of it, we say it doesn't own it
    // is not moved but referenced as follows
    let not_moved_value = String::from("Estoy siendo referenciado");
    let lenght_of_reference = referencing_variable(&not_moved_value);
    // and we can still using the variable
    println!("This is the value of not moved value: {not_moved_value}");
    println!("This is the lenght of the referenced value: {lenght_of_reference}");

    // by default references cannot be mutable even if the value of it is
    // try_to_mutate_a_reference_value(&not_moved_value);//this code should fail

    // we can mutate a reference value adding &mut to the function params
    let mut phrase = String::from("Hello");
    mutate_a_mutable_reference(&mut phrase);

    //restrictions to mutable references are that we cannot create more than one
    // we cannot do this
    let mut value1 = String::from("Value1");
    let _v1 = &mut value1;
    //This code should fail
    // let v2 = &mut value1;
    // println!("Borrowed values v1 and v2 are {}, {}", v1, v2);

    // so if we want multiple reference mutations we could do the next
    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;

    // we can also do this as long as we dont use r1 nor r2 after definig the r3 mutable reference
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    // println!("{} and {}", r1, r2);// we are not allowed to use unmutable references after
    // a mutable reference is defined

    // we cannot return a referenced value from a function cause is should be
    // a high cost operation cause it would keep that reference alive along all the
    // execution time
    // but this is in fact not posible cause the referenced value is out of scope
    // so it is no longer available when scope ends, so we reference to nothing
    // let reference_to_nothing = dangle(); // we are not allowed to do this

    // summary

    //     The Rules of References
    // Let’s recap what we’ve discussed about references:

    // At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.
}

fn borrowing_variable(moved_value: String) -> String {
    moved_value
}

fn referencing_variable(reference_value: &String) -> usize {
    // "reference_value" is just a reference to a value but not an ownership
    // so it doesn't contain a value by itself but references it
    // so it does not drop anything, just the stack that references the value
    println!("This is the value of not moved value: {reference_value}");
    reference_value.len()
}

// fn try_to_mutate_a_reference_value(s: &String) {
//     s.push_str("string");
// }

fn mutate_a_mutable_reference(s: &mut String) {
    s.push_str(", world!");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
