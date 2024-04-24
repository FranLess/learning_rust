fn main() {
    // Slice is an operation which interacts with the bytes of a complex variable
    let mut string = String::from("Slicing words of a sentence");
    // slicings are slice variable references and store the slice reference into another
    // this means that the content retrieved from the slice is still a reference
    // not an ownership nor borrowing

    let first_word = get_first_word(&string);
    // So we get the first word of the sentence but as it is a reference to a part fo the string
    // string should still exist/available if we'd like to use "fisr_word" variable

    // if we destroy/drop the content of the "string" variable we'll get an error
    // string.clear(); // we are not allowed do to this cause we are borrowing a mutable reference
    // so once we use a mutable reference to clear "string" we cannot use unmutable nor mutable references anymore
    println!("Firs word of {string} is: {first_word}");

    // As slices are actually string literals, with a static memory value
    // as we don do [..]
    // we are able to pass string literals into the function we've made

    let string = "Hola como estas"; // string literals refers to a unmutable string
    let first_word = get_first_word(string);
    println!("{first_word}");

    //EXAMPLES FROM THE BOOK
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = get_first_word(&my_string[0..6]);
    let _word = get_first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = get_first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = get_first_word(&my_string_literal[0..6]);
    let _word = get_first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = get_first_word(my_string_literal);
}

fn get_first_word(s: &str) -> &str {
    for (i, &byte) in s.as_bytes().into_iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
