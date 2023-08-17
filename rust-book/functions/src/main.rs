fn main() {
    println!("Hello, world!");
    another_function();
    function_with_parameter(5);
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("x is {x}");

    let x = plus_one(x);
    println!("x plus one is {x}");
}

fn another_function() {
    println!("Another Function!");
}

fn function_with_parameter(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}.");
}

// Implicitly Returning Function - IMP: NO SEMI-COLON at the end of the expression we want to return implicitly!
fn five() -> i32 {
    // Expression - auto returned in a function
    5
}

fn plus_one(x: i32) -> i32 {
    // IMP: NO SEMI COLON!!!
    x + 1
}
