#[allow(unused_mut, unused_variables)]

fn main() {
    println!("Collection - String!");

    // INFO: strings in rust is 2 bytes, instead of 1 byte

    // 1. Creating a new `String`
    let mut s = String::new();
    println!("s is {s}");

    // 2. Creating `String` from &str (string literal / string slice)
    let data = "This is a string literal";
    let data_str = data.to_string();
    println!("data_str is {data_str}");

    let data_str2 = "This is second string literal".to_string();
    println!("data_str2 is {data_str2}");

    // 3. Creating `String` using util function
    let str = String::from("Hello world!");
    println!("str is {str}");

    // 4. Updating the String
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // `push_str` DOES NOT take ownership of the parameter
    println!("s2 is {s2}"); // Still able to use s2 after passing to `push_str`

    let mut s = String::from("lo");
    s.push('l'); // to add single character
    println!("s is {s}");

    // 5. Contat Operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // 6. Format Macro

    // using `+`
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s (+) is {s}");

    // using `format!`
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s (format!) is {s}");

    // 7. Indexing into String

    let s1 = String::from("hello");
    // let h = s1[0]; // ERROR - indexing on string is not allowed as

    let hello = "Здравствуйте";
    // let answer = &hello[0]; // ERROR - this won't compile to prevent reading invalid bytes

    // Rather than indexing using [] with a single number,
    // you can use [] with a range to create a string slice containing particular bytes:
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 4 bytes / 2 (rust string char size) = 2 chars (Зд)
    // let s = &hello[0..1]; // ERROR - will panic at runtime -> 1 byte / 2 = 0.5 char - same error like [0] indexing
    println!("s is {s}");

    // 8. Methods for Iterating over String
    for c in "Зд".chars() {
        println!("{c}");
    }

    println!(">>> Зд [.bytes] <<<");
    for b in "Зд".bytes() {
        println!("{b}");
    }

    println!(">>> नमस्ते [.chars] <<<"); // Example
    for b in "नमस्ते".chars() {
        println!("{b}");
    }
}
