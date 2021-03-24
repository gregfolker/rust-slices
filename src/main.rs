// Project: slices
// Author: Greg Folker

fn main() {
	println!("Hello, World!");

    // Slices in Rust do not have ownership
    // Slices let you reference a contiguous
    // sequence of elements in a collection
    // rather than the whole collection at once
    let mut s = String::from("Hello, World!");

    let word = first_word(&s);

    println!("{}", word);
    s.clear();

    // At this point, `word` is completely invalid because
    // we cleared `s` above on Line 15. There is no more string
    // that we can meaningful use the value stored inside of `word` for,
    // but it is not a compiler error to keep referencing it because `word`
    // is not bound to `s` in any fashion
    println!("{}", word);

    // The solution to this potential problem where `word` can get out
    // of sync with the actual data in `s` is to use string slices
    // A string slice is a reference to a part of a `String`
    let s = String::from("Hello, World!");

    let hello = &s[0..5];
    let world = &s[7..12];

    println!("{} {}", hello, world);

    let word = first_word_slice(&s);

    println!("{}", word);

    // Now, the compiler is able to ensure the references into
    // `s` remain valid because a slice was taken from it
    // using `first_word_slice(&s)` on Line 35
    let mut s = String::from("Hello, World!");

    let word = first_word_slice(&s);

    // Allowed, because we have not modified `s` yet
    println!("{}", word);

    s.clear();

    // This is a compiler error now, because we are trying to use
    // an immutable borrow after the mutable borrow occured on Line 46
    // when we called `s.clear()`
    // println!("{}", word);

    // String literals are slices in Rust
    // The type of `s` is `&str`
    let s = "Hello, World!";

    println!("{}", s);


    // String Slices are specific to strings but you are still
    // able to make slices for any type you need to
    let a = [1, 2, 3, 4, 5];
    let _slice_of_a = &a[1..3];
}

// Returns the ending index of the first word in the provided String
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // Create an iterator over the array of bytes using the `iter()` method
    for (i, &item) in bytes.iter().enumerate() {
        // Search for the byte that represents the space by
        // using the literal syntax `b' '`
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Returns the first word in the provided string as a slice instead of an index
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
