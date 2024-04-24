use std::{collections::HashMap, ops::Add, path::Display};

//NOTE: String are actually very complex so this could not be clear enough to get
// to understand strings completly
fn main() {
    // Vectors can only contain data of one type
    let mut v = Vec::new();
    v.push(3);

    //another way to delcare vectors
    let mut v = vec![1, 2, 4, 5, 6];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // another way to access an index from vector without getting an error if index doesn't exists
    let third: Option<&i32> = v.get(2);
    if let Some(third) = third {
        println!("Third element is {third}");
    }

    let first = &v[0];
    v.push(7);
    // the code below shouldn't run
    // if let Some(third) = third {
    //     println!("Third element is {third}");
    // }
    // println!("The first element is: {first}"); // we are not allowed to borrow an inmutable reference after a mut reference

    // iterating through a vector
    for i in &v {
        println!("{i}");
    }

    // we are able to do this by referencig a value in vector
    // and then to deference the value to change the actual value in vector
    for i in &mut v {
        *i += 1;
    }
    // we should still being able to use &v cause mut reference went out of scope
    for i in &mut v {
        *i += 1;
    }

    // We are able to store different types using enums

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(123),
        SpreadsheetCell::Text(String::from("Hola")),
    ];
    {
        if let SpreadsheetCell::Text(string) = &row[1] {
            println!("Text in row {}", string);
        }
    } // we use this scope so we can still use references
    let text: &mut SpreadsheetCell = &mut row[1];
    println!("Text {:?}", text);

    //STRINGS
    // these ways to define strings do the same
    let s1 = "Hola"; // this line need the bellow to get the exact result as other lines
    let mut s1 = s1.to_string();
    let mut s1 = "Hola".to_string();
    let mut s1 = String::from("Hola");

    // we can contat to String like follows
    s1.push_str(" string");
    let s2 = "hola";
    s1.push_str(s2); // this just need references values

    // s1.push_str(&s1[0..4]);// this is invalid
    let s1 = s1 + s2; // this kind of expressions move complex values
                      // let sn = &s1 + s2; // This is not valid cause left term must be owned
    println!("s1 is: {s1}");
    println!("s2 is: {s2}");
    let s3 = format!("Hola {s1}"); // this doesn't take ownership of String type
    println!("s1 is: {s1}");
    println!("s3 is: {s3}");
    let s4 = String::add(s1, s2);
    // println!("s1 is: {s1}"); // s1 was moved
    println!("s2 is: {s2}");
    println!("s4 is: {s4}");

    //String are complex
    // the best way to iterate through and string is
    let string = "Hola";
    for c in string.chars() {
        println!("{c}");
    }
    // we might have a string which contains less characters than bytes like
    println!("WORD WITH MORE BYTES THAN CHAR");
    let hello = "Здравствуйте";
    for c in hello.chars() {
        print!("-{c}-");
    }
    println!();
    for c in hello.bytes() {
        print!("-{c}-");
    }
    // let c = "hello"[0]; // string cannot be indexed

    println!();
    println!();
    println!("-----MAPS--------");
    // Maps
    // There is no much to discut about MapHashes
    // Theyre just as on ohter languajes
    // We just need to consider all the ownership rules rust has
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // gets the value of insert 0 if key doesn't exists
                                                  // and returns the value of the entry
        *count += 1; // dereference the value of the key and increments it
    }

    println!("{:?}", map);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // Copied return a Option<T> of Copy values instead of a reference Option<&T>
                                                              // unwrap_or returns the value contained in option and if is None returns the value we passed

    let mut string_map: HashMap<String, String> = HashMap::new();
    string_map.insert(String::from("Hola"), "Hola".to_string());
    string_map.insert(String::from("Hola2"), "Hola".to_string());
    let value_one = string_map.get("Hola"); // This returns an unmutable reference

    // let value_one = string_map.get("Hola").copied(); // this is not valid cause would take ownerhsip of value and cannot be null

    // Though we understand the concept we should use more maps to get used to it
}
