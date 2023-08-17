fn main() {
    // 1. The `loop` 
    
    // a) Infinite loop!
    // loop {
    //     println!("again!")
    // }


    // b) Retruning values with break
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // c) Labeled Loops 
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // 2. While Loop
    let mut number = 3;
    
    while number > 0 {
        println!("number = {number}");
        number -= 1;
    }

    println!("While terminated with number = {number}");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // SLOW compared to for-loop due to compilier index bound checking at run time.
    while index < 5 {
        println!("a[{index}] = {}", a[index]);

        index += 1;
    }


    // 3. For loop
    
    for element in a {
        println!("The value is {element}");
    }

    // using range
    for number in (1..4).rev() {
        println!("count-down => {number}");
    }

    println!("Liftoff!!!")


}
