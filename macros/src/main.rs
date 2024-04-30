use hello_macros::HelloMacro;

// this code is kinda bugged, compiler sometimes interpretate
// comments as real code so it shows some warning or errors
// it happends inside the lib crate "hello_macros"
fn main() {
    // macros in fact are kinda difficult to understand
    // at least that's what I understood
    // there is an entire book that explains how to write macros
    // so I'll not explain how to write a macro
    // I think it is important to know Rust has the possibility
    // to write this kind of tools, so we don't desconsertate
    // if we see some in some point in the future
    // So I think trying to fully understand macros without have
    // written rust code, having practice building projects and stuff,
    // will be something that could not be quite useful
    let v = vec![1, 2, 3];
    for item in v {
        println!("{item}");
        Pancakes::hello_macro();
    }

    // we are able to write any kind of macro
    // for example macros to derive structs or enums
    // even some kind of @Annotations or @Decorators in other languajes
    // like in Java or C++ or C# where there are this kind of annotations
    // to get our code to do aditional things

    // We'll implement a derive macro for this
    #[derive(HelloMacro)] // see ./hello_macro to see
                          // this macro implementation
    struct Pancakes;

    pub trait HelloMacro {
        fn hello_macro(); // we now are able to use the macro
    }
}

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
