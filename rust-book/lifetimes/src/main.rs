use std::fmt::Display;
fn main() {
    println!("Hello, Lifetimes!");

    /*
     * 1. Preventing dangling reference
     */

    //  Error if complied
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {r}");

    /*
     * 2. Generic Lifetime anotation for compilier to know the reference will be valid
     * `result` will be valid until `string1` or `string2` (smallest life) goes out of scope!
     */

    // a. working example
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let result = longest(&string1, string2.as_str());
    println!("result: {result}");

    // b. non-working example (error)
    // let str1 = String::from("helloooo");
    // let result2;
    // {
    //     let str2 = String::from( "world");
    //     result2 = longest(str1.as_str(), str2.to_str());
    // }
    // println!("result2: {result2}");

    /*
     * 3. Lifetime anotation in struct defination
     */

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Behold the angle of The Lord. She was blessed.");
    let first_sentence = novel.split(".").next().expect("No `.` found!");

    let imp_expt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("important part: {}", imp_expt.part);

    /*
     * 4. 'static - lifetime for the entire life of the program!
     *
     * IMP: string literals have 'static lifetime by default!
     */

    let s: &'static str = "I have a static lifetime.";
    println!("s: {s}");

    /*
     * 5. Generic Type Parameters, Trait Bounds, and Lifetimes Together
     */
    let long_with_ann =
        longest_with_announcement(&string1, string2.as_str(), "Welcome to Lifetime");
    println!("long_with_ann: {}", long_with_ann);
}

/* Error if complied
  As the compiler can't figure out that the reference returned is for `x` or `y`
  See `src/notes.md` for more info.
*/
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
