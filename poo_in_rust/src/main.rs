use poo_in_rust::Post;
fn main() {
    // the the very new thing we've learned here is the Box<dyn U>
    // it is like a generic <T: U>, that determinates the type of T in compile time
    // and the Box<dyn U> determinates the type of
    //U in runtime instead. This means we coul use or return
    // diferent types that implements U cause they're not statically
    // determinated.
    // This is why a function like the following would work
    trait Animal {
        fn eat(&self); // even if we don't use self, we put it cause this makes
                       // our implementations to be related to this trait
                       // elsewhere implementations would be related to Animal
    }
    struct Dog;
    struct Cat;
    impl Animal for Dog {
        fn eat(&self) {
            println!("Guau guau I'm a dog eating")
        }
    }
    impl Animal for Cat {
        fn eat(&self) {
            println!("Miau Miau I'm a cat eating")
        }
    }
    // this would work cause compiler need to know which type sould return
    // statically, so it could only be 1 type not two or more
    // fn returning_some_animal_not_functional(index: u32) -> Box<impl Animal> {
    //     match index {
    //         1 => Box::new(Dog {}),
    //         2 => Box::new(Cat {}),
    //         _ => Box::new(Dog {}),
    //     }
    // }
    fn returning_some_animal(index: u32) -> Box<dyn Animal> {
        match index {
            1 => Box::new(Dog {}),
            2 => Box::new(Cat {}),
            _ => Box::new(Dog {}),
        }
    }
    returning_some_animal(2).eat();
    let mut post = Post::new();
    post.add_text("Hello how are you fine fine fine yeah yeah");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("Hello how are you fine fine fine yeah yeah", post.content());
}
