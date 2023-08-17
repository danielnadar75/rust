fn main() {
    println!("Ownership Exapmles!");

    // *** COPY vs MOVE Example ***
    // Move Example
    let s = String::from("hello");
    takes_ownership(s);
    // IMP: Cannot use s again here as it has moved in the function above

    // Copy Example
    let x = 5;
    makes_copy(x);
    println!("x from main function: {x}");

    // *** Returning Ownership example ***
    // 1. Taking a retun value from function
    let s1 = give_ownership();
    println!("s1:=> `{s1}`");

    let s2 = String::from("s2 is a String!");
    println!("s2:=> {s2}");

    let s3 = give_and_take(s2);
    println!("s3:=> {s3}");

    // 2. Taking multiple return values from function using tuple
    let (s4, len) = calculate_length(s1);
    println!("s4:=> {s4}");
    println!("s4 length: => {len}");

    // *** PASS BY REFERENCE ***
    println!("[PassByRef]");

    // 1. passing by reference - borrowing
    let s1 = String::from("hello s1 again! welcome back");
    let len = calculate_length_ref(&s1);
    println!("s1:=> {s1}");
    println!("s1 length: => {len}");

    // 2. passing by ref and trying to modify data -> ERROR (without &mut)
    let mut s = String::from("hello");
    // change(&s); // THIS WILL FAIL

    // 3. passing by ref and trying to modify data using `&mut` instead of `&`
    change(&mut s);
    println!("s:=> {s}");

    // This will cuase ERROR as 2 mut ref at same time!
    // let r1 = &mut s;
    // let r2 = &mut s;

    // THis will work fine - Scope is the way to go!
    {
        let r1 = &mut s;
        println!("r1:=> {r1}");

    }
    let r2 = &mut s;
    println!("r2:=> {r2}");

    // *** mut and immut ref at the same time - `NO ES POSSIBLE`!!!
    let mut s = String::from("hello");

    let r1 = &s; // No problem
    let r2 = &s; // No Proplem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{r1}, {r2}, {r3}");
    println!("{r1}, {r2} and r3 if we use here as mut will cuase error! ");

    let mut r3 = &mut s; // No problem -> as r1 and r2 are no longer used after this line!
    println!("r3: => {r3}");


    // *** Dangling References ***
    let ref_to_nothing = dangling();
    println!("ref_to_nothing (dangling)::::> {ref_to_nothing}");


    // *** Slice Type ***

    // 1. String <to> str
    let my_sent = String::from("hello to you too rust!");
    let first_word_index = first_word(&my_sent);

    println!("first word last character index: {first_word_index}");

    let first_word = first_word_slice(&my_sent);
    println!("first word : {first_word}");

    // 2. String/str <to> str
    let my_sent_str = "hello, world!"; // This works just like String type - Because str is a slice type of String type

    let first_word_str = first_word_slice_str(&my_sent_str);
    let first_word_str_slice = first_word_slice_str(&my_sent_str[..]);

    println!("first word from my sentance (string literal): {first_word_str}");
    println!("first word from my sentance (string literal slice): {first_word_str_slice}");

    let first_word_string = first_word_slice_str(&my_sent);
    let first_word_string_slice = first_word_slice_str(&my_sent[..]);

    println!("first word from my sentance (String Type): {first_word_string}");
    println!("first word from my sentance (String Type slice): {first_word_string_slice}");
   
    println!("This proves that String slice == &str OR &String = &str because they both are stored in binary as slice");

    // 3. Other slices (collection like array, etc)
    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[1..3];

    assert_eq!(a_slice, &[2, 3]); // Checks the values of array to equal
    // a_slice => [2, 3]
    // &[2, 3] => [2, 3]

    // Assert example
    let a= 3;
    let b = 1 + 2;
    assert_eq!(a, b, "We are checking if {a} == {b}"); // the message provided will only logged if it panic

    

}

// Move
fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

// Copy
fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn give_ownership() -> String {
    let str = String::from("Gave Ownership!");
    str
}

fn give_and_take(str: String) -> String {
    str
}

//  Pass My Value (Moving Ownership)
fn calculate_length(str: String) -> (String, usize) {
    let length = str.len();
    (str, length)
}

// Pass By Reference
fn calculate_length_ref(str: &String) -> usize {
    str.len()
}

// *** Trying to modify referenced/borrowed data ***

// 1. IMP: IT WILL THROW ERROR!
// fn change(str: &String) {
//     str.push_str(", world.");
// }

// 2. This will compile as it's mutable reference
fn change(str: &mut String) {
    str.push_str(", world!");
}


// ** Dangling Reference ***

// 1. Problem! -> as the s goes out of scope hence it's a ERROR
// fn dangling() -> &String {
//     let s = String::from("hello");
//     &s
// }

// 1. Solution -> if we return the string then it works as it changes just the ownership!
fn dangling() -> String {
    let s = String::from("hello");
    s
}


// *** Slice Type ***

// NO-Slice
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Slice
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word_slice_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// Notes on range
// [0..2]   == [..2]
// [2..len] == [2..]
// [0..len] == [..]