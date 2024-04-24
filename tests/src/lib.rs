pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // Tests don't pass if they panic
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    // assertions work just as in another languajes
    // when a test panics it doesn't pass cause of a failure in the
    // test itself
    // when a test doesn't assert means that test logic is right
    // but logic of code we are testing may be wrong
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // assert!(smaller.can_hold(&larger)); // this test shouldn't pass cause of this
        assert!(!smaller.can_hold(&larger)); // this test shouldn't pass cause of this
    }

    // assertions to equal values or unequal values
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_three() {
        // we assume that add_two could add three
        assert_eq!(5, add_two(2));
    }

    // passing aditional output to an assertion
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        ); // takes one parameter after the assertion for the message,
           // and parameters for the params inside the message, just like a print
    }

    #[test]
    // should panic will pass out test if it panics
    // we should also prove that our code does't run over
    // not desired scenarios
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")] // we can specify what message we expect in a panic call
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    // we are allowed to return Results types
    // so test will pass if return is Ok or failure if return is Err
    fn result_test() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    } // this test will fail|
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

// NOTE: go around tests path
// Notice that tests are common over lib.rs files, which means, library crates
// binary crates don't allow us to create tests folder, structure for binary crates
// are often src/main.rs and src/lib.rs
// main should only run the code needed by our project, so the compiler can compile it into
// a executable (.exe)
// src/lib.rs should be file containing our logic, our code
// we can divide our code into modules that are called from src/lib.rs
// but main.rs should still just call lib.rs

//NOTE: this is how i get how binary crates should be sctructured,
// it is important that we go and see other real projects so we can
// have a better idea of how to structure our project folders and files
// that's all for this cap