fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // in this case the
        // value of y will be shadowed cause in match patters
        // the value of a not defined variable is like
        // an any matchin
        Some(a) => print!("asdf"), // this is an unreachable code cause
        // we are already mathing a Some value with any value inside
        _ => println!("Default case, x = {:?}", x), // this mathing will be a Default
                                                    // so it will be executed even if
                                                    // we dont have a Some, just anything
                                                    // that we havent handle
    }

    println!("at the end: x = {:?}, y = {y}", x);

    //MUTIPLE MATCHING CASES
    let number = 2;
    match number {
        1..=10 => println!("Number is between 1 and 10"),
        11 | 12 | 14 => println!("Number is 11 or 12 or 14"),
        _ => println!("Number is not 11 nor 12 nor 14 and its not between 1 and 10"),
    }
    // this can also be used on chars
    let letter = 'c';

    match letter {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    //DESTRUCTURING STRUCTS
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p; // x and y now have the values inside x,y of p
    let Point { x: a, y: b } = p; // now we define a and b variables to hold x,y values of p

    match &p {
        // mathing structures with specific attribute values
        Point { x, y: 0 } => println!("On the x axis at {x}"), // if y is 0, x could be any i32 number
        Point { x: 0, y } => println!("On the y axis at {y}"), // if x is 0, y could be any i32 number
        Point { x, y } => {
            // if x and y are not 0 and could be any i32
            println!("On neither axis: ({x}, {y})");
        }
    }
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        // we can match and destructure an structure to access its values inside the match
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    // just more examples, destructuring could go as deep as we want over nested types
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
    // nested structures, it could be a struct containing tuples and destructure tuples inside
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    let Tupling {
        tuple1: (num1, num2),
        tuple2: (num3, num4),
    } = Tupling {
        tuple1: (1, 2),
        tuple2: (3, 4),
    };

    // matching if conditions
    let x = 10;
    let y = 5;
    match x {
        10 if y == 5 => println!("x is 10 and y is 5"),
        _ => println!("Default  value"),
    }
    let x = Some(10);
    let y = 10;
    match x {
        Some(n) if y == n => println!("n is equal to  n: {n}"),
        _ => println!("Default  value"),
    }

    //binding ranged data, ranged unknown specific value is binded to a variable using @ after a variable name
    let x = 10;
    match x {
        bind @ 1..=10 => println!("x is between 1 and 10 and its value is: {bind}"),
        _ => println!("Default matching pattern"),
    }
}
struct Tupling {
    tuple1: (i32, i32),
    tuple2: (i32, i32),
}
struct Point {
    x: i32,
    y: i32,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
