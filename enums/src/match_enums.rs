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
