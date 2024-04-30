use core::fmt;
use std::{ops::Add, path::Display};

fn main() {
    unsafe_code(); // Go to function definitions to see unsafe code section
    advanced_traits(); // Go to function definition to see advanced traits section
    advanced_types(); // Go to function definition to see advanced types section
                      // TODO write code example for advanced_closures_and_functions
    advanced_closures_and_functions();

    //NOTE: the section for macros is in another folder/project
    // because it is more complex and requires more than a file to
    // do it in the way the book shows
}
fn unsafe_code() {
    // Unsafe code
    // unsafe code let us do the next:
    //      - Dereference a raw pointer
    //      - Call an unsafe function or method
    //      - Access or modify a mutable static variable
    //      - Implement an unsafe trait
    //      - Access fields of unions

    // to access to a raw reference we do the following
    //      - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    //      - Aren’t guaranteed to point to valid memory
    //      - Are allowed to be null
    //      - Don’t implement any automatic cleanup
    let mut x = 5;
    let y = &mut x as *mut i32;
    let z = &x as *const i32;
    // to access data what raw pointers are pointing at we need to write unsafe code
    // I haven't got what raw pointers are, I think it is just pointing to a data
    // with a defined type and we don't care if is valid or not. We just want to access
    // a part of the memory where some data were saved either mut or not but.
    // that data could be still valid or not so we could be pointing a null value
    // which coul lead us to a segmentation fault/accessing restricted data

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    // split at mut is a function that internally uses unsafe code
    // so this means we don't strictly need to write unsafe code directly
    // we are allowed to delegate some unsafe code inside a function
    // and be able of accessing some unsafe code without having to directly
    // write an usafe code block
    // this doesn's guarantee save data as usual but keeps bugs and
    // undesired behaviour that may occur in asolated code blocks so we can
    // identify them easier
    let (a, b) = r.split_at_mut(3);

    //unsafe code block are also when we interact with externall code,
    // code/ could be functions written in another code languaje, so rust
    // cannot tell if that code is following the rules of the compiler
    extern "C" {
        fn abs(input: i32) -> i32;
    } // this is telling us that we are accessing to the standard library of languaje C

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    // we can also write rust code to be executed in another languaje, like C
    #[no_mangle] // this if for not overriding the name of this function
                 // cause compilers could set a weird name of this into another languaje
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C");
    } // we could import this into a C program and call it

    // mutatin an static variable (a variable with an static lifetime)
    // is also unsafe, cause this variables could live the entire execution of
    // our program, this means that we can have access to it from diferent threads
    // which could cause a "data race"
    unsafe {
        COUNTER += 1;
    }

    // essentially unsafe code blocks are for when we're trying to do something
    // which should work but compiler does not let us
    // unsafe code have their rules too, borrowing/ownerwhip rules still applies
    // but we can do more things, like accessing data of raw pointers.
    // Get to know where to write unsafe code and how to do it takes practice
    // so here we're just understanding what an unsafe code block means
}

fn advanced_traits() {
    // this section covers types in traits and Disambiguation
    // essentially

    // type in traits are almost just like Generics but using
    // types let us have just one implementation over the
    // undefined number of implementations we may have if we use generics

    trait Creature {
        type Sound;
        fn make_sound(&self) -> Option<Self::Sound>;
    }
    struct Minion {
        name: String,
    }
    impl Creature for Minion {
        type Sound = String;
        fn make_sound(&self) -> Option<Self::Sound> {
            Some(format!("{} says woof!", self.name))
        }
    }
    // this is not like a real scenario example but it ilustrates how to use a type
    let minion = Minion {
        name: "Julias".to_string(),
    };
    if let Some(sound) = minion.make_sound() {
        print!("{sound}")
    }

    // A real using
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // now disambiguation
    // this is for when we have multiple implementations of traits with functions
    // named the same
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }
    // We use the following notation to call each method
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // these last examples works only on implementations
    // that are related to its structure (the ondes that has self on their params)
    // so the following would work on the ones that does not have self on their params
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    let dog = Dog;
    println!("The baby name of the dog is: {}", Dog::baby_name()); // this should print Spot
    println!(
        "The baby name of an animal is: {}",
        <Dog as Animal>::baby_name()
    ); // This should print puppy

    // Requiring implementations of traits in another trait
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    impl OutlinePrint for Point {} // This could not be implemented if Display
                                   // would not
    impl fmt::Display for Point {
        // we are required to implement this trait
        // to implement OutlinePrint for Point
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // Another example of Display implementation
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    // Default generic type
    // as the name says is a default for a generic type just as follows
    struct Bar {
        some: i32,
    } // this means that by default our T generic will be i32
    trait SomeOther<T = i32> {
        fn greet(&self, generic_default_type: T) {
            println!("Hola");
        }
    }
    impl SomeOther for Bar {
        fn greet(&self, generic_default_type: i32) {
            println!(
                "Hola my some field is: {}. And my generic default type contains: {}",
                self.some, generic_default_type
            );
        } // if we are writing code
          // we can notice that if we use autocompletitions it
          //types i32 by default even if generic_default_type
          // is, as it says, a generic
    }
    let bar = Bar { some: 32 };
    bar.greet(12);
}

fn advanced_types() {
    // creating a custon type is common for some verbose types
    // if we'd like to accept some lengthy type in a function or any other site
    // we simplify that type by creating another

    struct Foo {
        // we notice that could tedious to write
        // this type if our code gets larger
        x: Box<dyn Fn() + Send + 'static>,
    }
    // we can create a type that is equivalent to Box<dyn Fn() + Send + 'static>
    // this simplifies our code and make it more readeable
    type EquivalentType = Box<dyn Fn() + Send + 'static>;
    struct Bar {
        x: EquivalentType,
    }

    // NOTE: code below is commented cause it makes the program panic
    // or run forever, concepts are good illustrated but you can
    // uncomment the code so see the behaviour

    // ! type
    // ! says that our code never returns anything, in other words
    // it means that an statement will never return a value/type
    // not even ()
    /*******erase the matching comment block so execute the code
        let y = loop {
        println!("Forever");
        // break 43; // we should return something in a break for a loop
        // if we want to recover something from the loop
        // if we dont break it, the value of a loop will be !
    };
    */

    // this applies for matching patterns if we panic in our code
    /*******erase the matching comment block so execute the code
    let x: Option<i32> = None;
    let nothing = match x {
        // this is type ! cause we panic every possible scenario
        None => panic!("This must not be empty!!!"),
        _ => panic!("Wow this isn't empty"),
    };
    */

    // and for the last, The Sized, this are types for the ones that
    // compiler Know at compile time
    // there is an special syntax for this trait that indicates
    // if A generic type is a size known at compile tipe
    fn generic<T: ?Sized>(t: &T) {}
    // &T is a reference cause if is not sized it means that value/data
    // will be allocated in the heap and the reference on the stack, which is
    // the reference to that data the one we are accesing
    // if we tell the compiler that a type could be or not sized
    // we must handle that type as a reference cause we don't know its size
}

fn advanced_closures_and_functions() {
    // we are able to pass functions into another function
    // parameter/s

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    let answer = do_twice(add_one, 5);
    assert_eq!(12, answer);
    println!("The answer is: {answer}");

    // we can apply some method, as we did in disambiguation for those where aplicable,
    // to some value that we can accept inside the method
    let v = vec![1, 2, 3, 4, 5];
    let v_string: Vec<String> = v.iter().map(ToString::to_string).collect();
    v_string.iter().map(|string| println!("{}", string));
    // we can even create types of an enum if we'd like to
    enum SomeType {
        NumberType(u32),
        AnotherType,
    }
    let v_some_type: Vec<SomeType> = (0u32..20).map(SomeType::NumberType).collect();
    let v_string = vec!["string1", "string2"];

    // We can also return a closure as a function from a function using an smart pointer,
    // usually Box<T>
    fn return_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x: i32| x + 1)
    }
    println!("{}", return_closure()(5)); // This should print 6
                                         // there could be a lot of applications for this kind of writing closures
                                         // practice and time is the only thing that will take us to understand this much better
}
static mut COUNTER: i32 = 0;
