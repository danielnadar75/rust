use std::io;

fn main() {
    // ----------- Variables -----------
    // Imutable
    let y = 10;
    println!("The value of y is {y}");
    // y = 20; // This will throw error!

    // Mutable
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // ----------- Constants -----------
    // Static
    const NUMBER_OF_MONTHS_IN_A_YEAR: u8 = 12;
    println!("Constant - NUMBER_OF_MONTHS_IN_A_YEAR is {NUMBER_OF_MONTHS_IN_A_YEAR}");

    // Computed
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constant - 3 Hours In Seconds: {THREE_HOURS_IN_SECONDS}");

    // ----------- Shadowning -----------
    // Temp Re-Assignment
    let z = 5;
    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is {z}");
    }
    println!("The value of z in outer scope is {z}");

    // Datatype changing
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The space count is {spaces}");

    // ----------- Data Types -----------

    // 1. Example
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Your guess is {guess}");

    /* Possible Error If we don't explicitly specifiy type above for guess:
     *
     *    let guess = "42".parse().expect("Not a number!");
     *       ^^^^^
     * help: consider giving `guess` an explicit type
     * let guess: /* Type */ = "42".parse().expect("Not a number!");
     */

    // 2. Scalar Types:
    /*
     * a. integer
     * b. floating-point number
     * c. boolean
     * d. character
     */

    // a) Integer
    let int_a = 128;
    println!("Default Integer is i32 as you see for {int_a}");

    let int_a = 58u8;
    println!("{int_a} - is a integer literal with explicitly type annotation!");

    let int_large_num = 100_000_000;
    println!("{int_large_num} - we can also use _ to split a long number!");

    // b) Floating Point
    let float_a = 2.0;
    println!("Default float is f64 (double precision) as you see for {float_a}");

    let float_b: f32 = 9.99;
    println!("The coke is only for ${float_b}. (single precision)");

    // Math operators
    let sum = 5 + 10;
    println!("Sum is {sum}");

    let difference = 95.5 - 3.4;
    println!("difference is {difference}");

    let product = 4 * 30;
    println!("product is {product}");

    let quotient = 57.5 / 32.6;
    let truncated = -5 / 3;
    println!("quotient is {quotient}");
    println!("truncated is {truncated}, integer division will truncate the value!");

    let remainder = 43 % 5;
    println!("remainder is {remainder}");

    // c) Boolean
    let t = true;
    let f: bool = false;
    println!("t is implicitly typed with value: {t}");
    println!("f is explicitly typed with value: {f}");
    println!("The size of boolean is 1 bytes.");

    // d) Character
    let c = 'C';
    let z: char = 'Z';
    let black_heart = '🖤';

    println!("c is {c}");
    println!("z is {z}");
    println!("black heart: {black_heart}");
    println!("Char is 4 bytes in rust unline other programming languages & can store lot more than just ASCHII values!");

    // 3. Compound Types

    // a) Tuple - Collection of fixed size, different types of elements
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructuring with pattern matching
    let (a, b, c) = tup;
    println!("a: {a} b: {b} c: {c}");

    // using period (.) to access the elements
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("Values using (.) : {five_hundred} | {six_point_four} | {one}");

    let _unit = ();
    println!("This is a special type called `unit` and it is created with empty tuple");

    // b) Array - Collection of fixed size, similar types of elements
    let arr_1 = [1, 2, 3, 4, 5];
    println!("Arrys are allocated on stack instead of heap!");

    let first = arr_1[0];
    println!("arr_1 is implicitly typed, first is {first}.");

    let arr_2: [u8; 5] = [1, 2, 3, 4, 5];
    println!(
        "arr_2 is explicitly typed with length, arr_2[2] is {}.",
        arr_2[2]
    );

    let arr_3 = [100; 5]; // initializing with 100 at each index
    println!(
        "arr_3 is filling array with value for length, arr_3[4] is {}.",
        arr_3[4]
    );

    // accessing index out of bound
    invalid_array_indexing();
} 

fn invalid_array_indexing() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number!");

    let element = a[index];

    println!("The value of an element at index {index} is: {element}.")
}
