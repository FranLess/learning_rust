fn main() {
    // Closures are something like arrow functions in other languajes
    // closures often infer types from the enviroment, they rarely
    // express types on their boty though they can

    let closure1 = |arg: i32| arg + 1;
    let closure1 = |arg| arg + 1; // when closures are not typed
                                  // they need to infer it from the enviroment/context
                                  // so we need to call it at least once

    closure1(32); // if we dont write this our closure couldn't infer the type of the parameter

    // closures often just borrow ownerships from enviroment either mut or not
    let text = String::from("Hola");
    let borrowing_closure = || println!("{text}");
    borrowing_closure();
    let mut vector_list = vec!["Hola"];
    println!("{:?}", vector_list);
    let mut borrowing_mut_closure = || vector_list.push("value"); // here mut reference is defined
                                                                  // so we cannot use any other mutation after we use the closure
                                                                  // after we call it, mut reference will no longer be valid so we can take more borrowings

    // println!("{:?}", vector_list);
    borrowing_mut_closure(); // as FnMut closures can be called more than once
                             // compiler interpretates that mutable refence is created till the last call to it

    println!("{:?}", vector_list);
    // borrowing_mut_closure(); // if we call this, we'll not be able to create any reference
    // to vector_list before

    let mut text = String::from("Hola");
    // this is FnOnce cause it takes, change, ownerwhip of a value of the enviroment/context
    let fnonce_closure = || -> String { text + " hello" };
    fnonce_closure();
    // We cannot call fnoce closures more than once
    // fnonce_closure();// this is not allowed

    // Examples of closures
    // This closure is mean to be FnOnce but, even we pass a Fn closure, we can call any other
    // Fn type cause we still be able to call it even if we can call it more than once
    let optional = Some("Hola").unwrap_or_else(|| "Not found value of option"); // this closure takes no params
                                                                                // cause none is like a null value

    struct Rectangle {
        width: u32,
        height: u32,
    }
    let mut sorting_list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // FnMut that can be called more than once
    sorting_list.sort_by_key(|r| r.width);

    //ITERATORS
    let v1 = vec![1, 2, 3];
    let v2 = v1.iter().map(|x| x + 1);
    println!("{:?}", v1);


    //NOTE: as number of concepts, methods and types, are increasing
    // I recomend refering to the docs if any doubt, we just need to understand
    // how thinks work, which is explained in the book. We should try to memorize everything
    // we can refer to the docs if want to know more specific behaviour of something
    // just as iterators and its methods this time, as why map doesn´t take ownership
    // of the iterator where is called?


    // ANOTHER note: next part of this chapter foucuses on improving the minigrep project
    // so go there to see changes
}
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

// part of the example
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

// This is a good example for closures use
fn example() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
