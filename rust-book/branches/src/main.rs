fn main() {
    let number = 6;

    // If - Else
    if number < 5 {
        println!("condition was true!");
    } else {
        println!("condition was false!");
    }


    // If - Else If - Else
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2.")
    }

    // If with "let"
    let condition = true;
    let number = if condition { 5 } else { 6 }; // IMP: All blocks should return the same datatype!

    println!("the value of number with conditional let is {number}");
}
