use proc_macro::TokenStream;
use quote::quote;

// ------------ MACRO IN THE WAY OF #[derive(Macro)]
#[proc_macro_derive(HelloMacro)] // proc macro tells the compiler this will be a macro
                                 // after proc_macro comes the type of macro it will be
                                 // this time it is derive macro, cause we'll apply it in
                                 // the #[derive(Macro)] way
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // This is like the reference of the Macro name, which is
    // the structure name something like the className in Java,
    // the name of the structure
    let name = &ast.ident;
    // this generates the macro (implements a default for any structure we pass to derive())
    let gen = quote! {
            // This creates an implementation for the structure we are manipulating
    // its the default way to do it but having #name which is like a reference for
    // the structure that we want the implementation be available
            impl HelloMacro for #name {
                fn hello_macro() {
                    println!("Hello, Macro! My name is {}!", stringify!(#name));
                }
            }
        };
    gen.into()
}

// -------------- MACRO IN THE WAY OF ANNOTATION #  [some_annotation]

// here we just have an example of how a macro of this type could be defined
// as I said in the main file of this project
// it seems like macros are complex, so I think it's better to get used
// to write rust code and then, once we are acquainted enough to rust, we could
// start writing macros, may be of things we'd like to automate
// like some code we found really verbose and could be simplyfied
// by creating a macro

// The way to define an annotated/attribute macro is
// writing #[proc_macro_attribute]
// we won't go into this much deeper cause of what I wrote up here
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

// FUNCTION LIKE MACROS
// another way to define macros, that is really similar to the one we created
// in the main file of this project, is the following way
// this is for when we could have a complex implemtation of a macro
// that we'd get to write in a better way by using a declarative function
// to handle it, just as follows

// *Code example of the book*
// let sql = sql!(SELECT * FROM posts WHERE id=1);
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {
