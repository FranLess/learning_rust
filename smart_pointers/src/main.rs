use std::{
    borrow::Borrow,
    ops::Deref,
    rc::{Rc, Weak},
};

fn main() {
    // Box<T> pointer
    // It is just used to desviate the reference of a value to ohter place in heap
    // this means it will create a reference to a value on the heap
    // a useful scenario, if we know how to use Cons Lists
    // would be deferencing infinite  nested Lists in a Cons list

    // enum List{ // This is detected by compiler like Infinite size
    //     // cuause it contains a type which contain a i32 an istelf, which causes
    //     // a infinite Nested Lists
    //     Cons(i32, List);
    //     Nil,
    // }

    // As we don't want to have infinite nested data
    enum List {
        // This case Box<List> change the location of the reference of List
        // in Cons(i32, List) for the List to allocate in ohter direction
        // which not be inside, so we don't have a type that increments its size
        // inifinitely.
        // Technically it is still infinite but not as a single value, like a
        // list which contains Lists outside.
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};
    // This is actually kinda hard to read
    // but in fact are nested values/tuples

    let cons_list = List::Cons(12, Box::new(Cons(23, Box::new(Cons(12, Box::new(Nil))))));

    // IMPLEMENTATION OF DEFERENCES
    // * is the deference operator and it calls T.deref()
    // method on those that implements Deref trait
    // let's implement it

    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            Self(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // now we actually can call the value contained in the noun named value of MyBox
    // by using *
    let x = 5;
    let z = &x;
    let y = MyBox(x);
    assert_eq!(x, *y);
    assert_eq!(x, *z); // just as we would compare a dereference of x
                       // assert_eq!(x, z); // we cannot compare references to owned types, cause are diferent types

    // NOTE: it's not clear the actuall use of implementing Deref, so diving on more
    // examples could be useful

    // DROP trait
    // Drop trait let us define a method (drop) which will be called at the time
    // our instance of out type go out of scope/be disposed
    struct CustomDataType {
        data: String,
    }
    impl Drop for CustomDataType {
        fn drop(&mut self) {
            println!("The custom type with data: {} went out of scope", self.data)
        }
    }

    {
        let custom_data_type = CustomDataType {
            data: String::from("hola hola"),
        };
        let custom_data_type2 = CustomDataType {
            data: String::from("hola hola pero 2"),
        };
    } // Here the drop method will be called
      // order of droping memory is inverse of what they are defined
      // custom_data_type2 will be dropped first
    println!("Custom data type went out of scope");

    // Rc Referece Counted Smart Pointer
    // this pointer basiclly makes inmutable references
    // but this references are owned references, which means
    // that they don't depend on the first defined value of a memory allocation
    // the data allocated will sill remain until last of the owned references
    // be valid

    #[derive(Debug)]
    enum List2 {
        Cons2(i32, Rc<List2>),
        Nil2,
    }
    // impl Drop for List2 {
    //     fn drop(&mut self) {
    //         if let Cons2(number, list) = self {
    //             println!(
    //                 "Reference counter went out of scope with cons list: ({}, {:?})",
    //                 number, list
    //             );
    //         }
    //     }
    // }
    use List2::{Cons2, Nil2};

    let mut b: List2;
    {
        let a: Rc<List2> = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let c: List2 = Cons2(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        b = Cons2(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        let d: List2 = Cons2(14, Rc::new(Cons2(2, Rc::clone(&a))));
        println!("count after creating d = {}", Rc::strong_count(&a));
    }
    // if let Cons2(number, list) = &b {
    // THis is just a lab code, I was playin to try to get this better
    // }
    println!("Reference of b after a went out of scope = {:?}", b);
    // a were still in the scope even if it was defined in the same scope as c
    // it went out of scope the last, this proves reference value still until all owned references
    // are not valid anymore
    // Rc<List2> goes out of scope first and then List2 contained in there

    //NOTE: again, the behaviour of smart pointer tend to be complex
    // so it is also recomended to get acquainted to these reading more examples

    // Combination of Rc and RefCell
    /*asdfasd */

    #[derive(Debug)]
    enum List3 {
        Cons3(Rc<RefCell<i32>>, Rc<List3>),
        Nil3,
    }

    use std::cell::RefCell;
    use List3::{Cons3, Nil3};

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons3(Rc::clone(&value), Rc::new(Nil3)));

    let b = Cons3(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons3(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // what this let us do is to mutate values inside Rc
    // so we can save owned references and mutate them
    // as I understand, we can have multiple owned immutable references
    // and we can mutate any of them any time we want
    // following the borrowing rules

    *value.borrow_mut() += 10; // this borrow muts go out of scope inmediatly cause not saved into variables

    let mut x = value.borrow_mut(); // as we borrow mut this value and compiler doesn't know
                                    // we need to drop it to print it cause we need an immutable borrow
                                    // if we try to print a borrowed mut value we'll se "borrowed"
    if let Cons3(number, list) = &b {
        // so we can actually borrow the a value again
        if let Cons3(number, list) = list.borrow() {
            // let mut y = number.borrow_mut(); // defining another mut reference for
            //                                  // the same value in "x"

            // *y += 10;
            *x += 10 // we shouldn't be allowed to do this cause 2 mutable references remain valid
                     // but compiler doesn't this so we'll get a panic in runtime
        }
    }
    drop(x);
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // NOTE: Idk how tf i got this concept this fast, only like 1 hour lol, this is hard abstraction

    #[derive(Debug)]
    struct Parent {
        children: RefCell<Vec<Rc<Child>>>,
    }

    #[derive(Debug)]
    struct Child {
        parent: Weak<Parent>,
    }

    let parent = Rc::new(Parent {
        children: RefCell::default(),
    });

    let child = Rc::new(Child {
        parent: Rc::downgrade(&parent),
    });
    let child2 = Rc::new(Child {
        parent: Rc::downgrade(&parent),
    });
    {
        let mut children = parent.children.borrow_mut();
        children.push(child.clone());
        children.push(child2.clone());
    }
    println!("Parent {:?}", parent);
    println!("Child 1 parent {:?}", child.parent.upgrade());
    let child2 = child2.parent.upgrade();
    println!("Child 2 parent {:?}", child2);
    if let Some(parent) = child2 {
        println!("Parent of child2: {:?}", parent);
        let child2 = &parent.children.borrow()[0];
        let child2 = child2;
        if let Some(parent) = child2.parent.upgrade() {
            println!("Parent of child2: {:?}", parent);
            let child2 = &parent.children.borrow()[0];
            let child2 = child2;
        }
    }
    let count = Rc::strong_count(&parent);
    println!("references to parent: {}", count)
    // this is actually confusing
    // setting references doesn't create bucles
    // this references will just conain the reference even if they're nesten they will no overflow
    // cause number of references don't increment so memory doesn't, memory usage remains bounded,
    //in fact references will always stay the same
    // not increasing but referecing the same over and over
}
