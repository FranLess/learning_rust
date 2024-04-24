use rand::Rng;
// NOTE: This project seems like takes two diferent topics
// But match statements and Enums are related cause are match
// are used check what a variable containig an Enum value 
fn main() {
    // Enums in rust are just like any other languaje

    // for the Message enum we can make this
    let message = Message::Move { x: 5, y: 8 };
    message.call(); // NOTE: not sure about how to use this
    let message = Message::ChangeColor(0, 0, 0);
    message.call();
    // I would describe an enum like a collection of structures
    // A collection that can give a method to all of its contained structures

    // for the ips example we can
    let ip4 = IpAddr::V4(Ipv4Addr {});
    let ip6 = IpAddr::V6(Ipv6Addr {});
    // this needs more practice to know useful cases

    // One helpful type of enum is Option that is in the standard library of rust
    let optional_number = Some(123);
    let optional_string_literal = Some("Hola mundo");
    let optional_string = Some(String::from("Hola mundo"));
    let null_value: Option<i32> = None; // We need to define manually the type
                                        // cause rust doesn't know which type a None value should be

    // As told, this need more practice to get to see the use cases

    // go to definition of the function to understand concepts involved here
    match_enums();

    //matching
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // we gotta match each possible value or do something on non handled value
    let dice = 7;
    match dice {
        4 => do_dice_stuff(4),
        other => handle_other(),
    };
    fn do_dice_stuff(n: i32) {};
    fn handle_other() {};

    // we can also indicate to ignore any not handled value
    let value = 2;
    match value {
        1 => do_stuff(),
        _ => (),
    }
    fn do_stuff() {}

    // finally we can match Option values
    let option = Some(3);
    match option {
        Some(s) => do_stuff(), // s is the actuall value contained in Option type
        None => do_another_stuff(),
    }
    fn do_another_stuff() {}

    // we can use if as a not complete match statement
    // if we get some value not known from any function
    #[derive(Debug)]
    enum Color {
        Black,
        Blue,
        White,
        Red,
    }
    fn random_enum_value() -> Color {
        let n = rand::thread_rng().gen_range(0..=3);
        match n {
            0 => Color::Black,
            1 => Color::Blue,
            2 => Color::Red,
            3 => Color::White,
            _ => Color::Black,
        }
    }
    let color = random_enum_value();

    // so for this cases we could use a match statement but we can also use
    // if let if we just want to check over a single type of value
    if let Color::Blue = color {
        println!("The color is {:?}", color);
    } // this is the same as doing the match
    match color {
        Color::Blue => println!("The color is {:?}", color),
        _ => (),
    }
}

// We can define enums as follows
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
} // We can notice that enums essentialy contain
  // structure types

// we could define the following
// contained in Message enum structures
struct Quit;
struct Move {
    x: i32,
    y: i32,
}
struct Write(String);
struct ChangeColor(i32, i32, i32);

// NOTE FOR ME: I've not understood in what cases
// enums are like really usefull, just practice more

// we are actually able to implement methods on enums
// just as on structures
impl Message {
    fn call(self) -> Self {
        self
    }
} // Here, "self" will contain the value type of what
  // the enum we call on it contains
  // if we call it on a Move value, self will be Value type
  // and so on

//Another example given in the book is the following kind of IPs

struct Ipv4Addr {}
struct Ipv6Addr {}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn match_enums() {
    let coin_value = Coin::Quarter(UsState::Alabama);
    let value: u8 = value_in_cents(&coin_value);
    println!("The value in cents of coin is {value}");
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// we can match type of value we contain into a
// variable of type Coin
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);
            25
        }
    }
}
