use std::{
    fmt::{format, Display},
    iter::Sum,
};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // Generics on enums or structs
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let p_with_sum = Point { x: 20, y: 20 };
    let x = p_with_sum.x;
    let _sum = p_with_sum.sum(); // we are able to use sum cause x, y are both i32

    // this is getting more complex so pay attention
    // as this is getting more complex to remember I think it is neecessary to
    // practice get used to scenarios that contain this concepts to understand them better

    // Traits
    // Traits have a similar behaviour as interfaces in other languajes, like Java, PHP, CSharp
    trait Summary {
        fn summarize(&self) -> String; // this is similiar to a interface declaring a method
                                       // we instead could create a default behaviour for a trait
        fn sumarize_author(&self) -> String {
            format!("{} \t Read more in author page...", self.summarize())
        } // we are able to call defined methods even if they dont have default behaviour
          // but defining a default behaviour doesn't mean that we cannot implement custom implementation of the method
          // we're still being able to override it, just as interfaces in other languajes
    }
    // now we can implement it over a structure/enum
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
        fn sumarize_author(&self) -> String {
            format!(
                "{} \t Read more in author page {}...",
                self.summarize(),
                self.author
            )
        }
    }
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // this will allow us to do similar things as follows
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    // this could be either a tweet or a NewsArticle
    println!("1 new tweet: {}", tweet.summarize());

    // trait let us define generics which must implement traits
    fn summarizable_params(x: &impl Summary, y: &impl Summary) {
        x.summarize();
        y.sumarize_author();
        // Notice that we can call implemented methods of the trait
    }
    // another way to write this, most known in other languajes, is
    fn summarizable_params2<T: Summary>(x: &T, y: &T) {
        x.summarize();
        y.sumarize_author();
        // Notice that we can call implemented methods of the trait
    }
    // we are also able to return generic value of and implemented trait
    // but we are only allowed to return one type of the implemented trait
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
    // our code will panic if we try this
    // uncoment the function
    // fn returns_summarizable(switch: bool) -> impl Summary {
    //     if switch {
    //         NewsArticle {
    //             headline: String::from("Penguins win the Stanley Cup Championship!"),
    //             location: String::from("Pittsburgh, PA, USA"),
    //             author: String::from("Iceburgh"),
    //             content: String::from(
    //                 "The Pittsburgh Penguins once again are the best \
    //                  hockey team in the NHL.",
    //             ),
    //         }
    //     } else {
    //         Tweet {
    //             username: String::from("horse_ebooks"),
    //             content: String::from("of course, as you probably already know, people"),
    //             reply: false,
    //             retweet: false,
    //         }
    //     }
    // } // Chapter 17 will discuss more about this sort of behaviour

    // for the last of traits, we are able to write "Blanket implementations"
    // this means that we can write implementations that will be available only
    // for those instances that matches types specified, just as follows
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // Notice that this method will only be available for those
    // instances that has a type which implements Display and PartialOrd traits
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // Now, for the last, Lifetimes
    // This is kinda simple
    // lifetimes only indicate how long params or attributes should live
    // for those to work
    // another way to say is, lifetimes indicates that variables with one
    // specific lifetime should live all together for the logic we write works
    {
        // this a representation of how compiler interpretate lifetimes
        let x = 5; // ----------+-- 'b
                   //                     |
        let r = &x; // --+-- 'a     |
                    //              |     |
        println!("r: {}", r); //    |     |
                              // --+      |
    } // ----------+

    // the way we define a lifetime is within <'x>
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    } // this function is basically telling that will return a reference
      // that is valid as long as x and y still alive/valid
      // and x and y should be valid/in the scope when we use the returned reference

    let string1 = String::from("long string is long");
    let mut result: &str;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    } // we're allowed to do this cause string1 and string2 are still valid when we use result

    // *uncomment
    // println!("The longest string is {}", result); // we are not allowed to do this
    // cause string2 went out of scope/is not valid here
    // and so result isnt

    // lifetimes allow us to take referenced struct attributes
    struct ImportantExcerpt<'a> {
        part: &'a str,
    } // this tell us that any instance of ImportantExcerpt
      // will be valid as long a the reference taken in part be valid too

    let mut i: ImportantExcerpt;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("Part of importantExcerpt is: {}", i.part)
    } // this works fine cause here first_sentence and novel are living as long as i
      // println!("Part of importantExcerpt is: {}", i.part) // we are not allowed to use i here
      // cause novel went out of scope which should has same lifetime as i

    // we can do the same but with novel out and first_sentence outer so they live enough
    let mut i: ImportantExcerpt;
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    {
        i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    println!("Part of importantExcerpt is: {}", i.part);

    // static lifetime
    let s: &'static str = "I have a static lifetime."; // all string literals are static lifetimed
                                                       // I cannot really explain a practical scneario for this

    // static just means that lifetime is valid for the entire scope it is defined
    // in comparative with definitions of 'a which tell us how long related variables should be valid at the same time
    println!("Static HOLA is: {}", HOLA);

    // and for the last an example of almost everything covered in this chapter (10)/project
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    //NOTE: i recomend reading the book cause this is like a practical summary, book contains much more info
    // or could also ask GTP for explanations, maybe
}
static HOLA: &str = "HOLA";

// as I don't want files get large I'll just write examples without
// writing less abstract versions

// A function using generics for getting largest value
// in a list/vector of values which implements the PartialOrd trait

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for number in list {
        if largest < number {
            largest = number;
        }
    }
    largest
}

pub struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// If we write implementation without generics the implementations we write will only be aplied
// for the instances which match the types we pass into the structure/enum

// for example, we'll be able to use this method only if values contained in Point are both i32
impl Point<i32, i32> {
    fn sum(&self) -> i32 {
        let sum = &self.y + &self.x;
        sum
    }
}

// monomorphized means that the compiler actually creates versions of enums or structures that uses generics
// the versions are versions which contains specified values for values we use on generics
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn monomorphized() {
    // this two definitions would be internally the same as the two following
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
    // This are to the compiler similar to the ones over
    // Compiler uses other names but esentially is the same behavour
    let integer = Option::Some(5);
    let float = Option::Some(5.0);
}
