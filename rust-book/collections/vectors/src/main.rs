#![allow(unused_variables, unused_mut)]

fn main() {
    println!("Hello, collection::vectors!");

    // **  vectors ***
    // - Vectors are like array - collection of similar data types!
    // - Stored in continues memory location
    // - Different types of data can be stored using enum (see point-5 for details)

    // 1. Defining using `new`
    let mut v: Vec<i32> = Vec::new(); // here we need to defined Vec<T> type as compilier can't figure out

    // 2. Defining using macro
    let mut v2 = vec![50, 60, 70]; // here we dont need to define type explicitly

    // 3. Operations
    v.push(10);
    v.push(20);

    let first = &v[0];
    // v1.push(30); // ERROR -> can't hold `first` and do operation on v1 at the same time!

    println!("first element is {}", first);

    v.push(30); // THis is valid as first is no longer used after line 18!

    // 4. Looping vector

    // a. immutable
    for i in &v {
        println!("{i}");
    }

    // b. mutable
    for i in &mut v {
        // NOTE: we need to dereference to do the arthematic operation as we need to get the value first from the memory!
        *i += 5;
    }

    // 5. Using enum to store multiple types in a vector!
    #[derive(Debug)]
    enum SpreadSheetData {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetData::Int(100),
        SpreadSheetData::Float(34.5),
        SpreadSheetData::Text(String::from("London")),
        SpreadSheetData::Int(200),
    ];

    for col in &row {
        match col {
            SpreadSheetData::Int(val) => println!("int found {}", val),
            _ => println!("other")
        }
        // println!("{:?}", col);
    }

    // 6. Vector is scoped - memory is freed/deleted once it goes out of scope
    {
        let v3 = vec![10];
    } /* v3 goes out of scope here */
}
