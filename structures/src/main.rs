fn main() {
    // structures in rust are something similar to classes in other languajes
    let rect1 = Rectangle {
        // this is the way to instantiate a structure
        height: 12,
        width: 4,
    };

    // we can create duplicated instaces of a structure by destructuring another
    let mut rect2 = Rectangle {
        height: 12, // this is for specific value
        ..rect1     // this will take values that are not specified before
    }; // but would take ownership of rect1 if complex data is stored
       // this case doesn't take ownership cause of simple data stored
       // we could try creating instances and passing references but we need
       // to manage lifetimes which we are not acquainted to yet
       // let rect3 = Rectangle { ..&rect2 };

    // we are allowed to mutate attributes as long as we create a mutable instance
    rect2.height = 100;

    //THE WAY TO PRINT STRUCTURES
    println!("The rect2 is {:?}", rect2); // doing prints this way will require
                                          //us to anotate the structure with #[derive(Debug)]

    //ANOTHER WAY TO PRINT STRUCTURES

    let _rect2 = dbg!(rect2); // debug prints all kind of expressions but
                             // it takes ownership of what we pass to it,
                             //so we need to assign it to a variable

    // we can use what we've just build to do this
    let rect1 = Rectangle {
        height: 12,
        width: 30,
    };
    let rect2 = Rectangle {
        height: 15,
        width: 30,
    };
    let rect3 = Rectangle {
        height: 10,
        width: 10,
    };
    println!("rect1 can hold rect2 {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3 {}", rect1.can_hold(&rect3));
}
//Definition of a structure
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

// implementations of a structure are for defining methods of it
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

// we are able to implement as many methods as we want
// and if our method doesn't have self param, it is a "static"
// these type of methods are called "Associated functions" in rust
impl Rectangle {
    fn square(side: u32) -> Self {
        Self {
            height: side,
            width: side,
        }
    }
}

//we can create structures in the tuple way
struct Color(u32, u32, u32);
//which shoul be created as follows
fn create_tuple_structure_way() {
    let _black_color = Color(0, 0, 0);
}

// we can also can create structures which contain unit type "()"
struct SingleType;
