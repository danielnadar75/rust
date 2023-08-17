enum IpAddrKind {
    V4,
    V6,
}

struct IpAddress {
    kind: IpAddrKind,
    addr: String,
}

enum IpAddr_1 {
    V4(String),
    V6(String),
}

enum IpAddr_2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}
/*
Explanation of above type:
==========================
struct QuitMessage; // unit
struct struct MoveMessage {     x: i32,     y: i32, }
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
 */
impl Message {
    fn call(&self) {}
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("Hello, Enum!");

    // *** Defining Enum ***

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
    route(IpAddrKind::V4);

    // *** using struct with enums ***

    let home = IpAddress {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: IpAddrKind::V6,
        addr: String::from("::1"),
    };

    // *** using enum with values! ***
    let home = IpAddr_1::V4(String::from("127.0.0.1"));
    let loopback = IpAddr_1::V6(String::from("::1"));

    // *** using enum with different types! ***
    let home = IpAddr_2::V4(127, 0, 0, 0);
    let loopback = IpAddr_2::V6(String::from("::1"));

    // *** example of multi-type and impl on enum to define methods ***
    let msg = Message::Write(String::from("rusty msg!"));
    msg.call();

    // *** Rust's favorite `Option<T>` type! ***
    let some_number = Some(5);
    let some_char = Some('a');
    let absent_number: Option<i32> = None; // explicit type needed as compiler can't figure out the <T> from `None`

    // operation
    let x: i8 = 5;
    let y: Option<i8> = Some(1);

    // let sum = x + y; // ERROR - cannot add `Option<i8>` to `i8`

    // *** The match Control Flow Construct ***
    let coin = Coin::Penny;
    match coin {
        Coin::Penny => {
            println!("Hey! Found a penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };

    value_in_cents(coin);

    // *** Patterns That Bind to Values
    let coin2 = Coin2::Quarter(UsState::Alaska);
    match coin2 {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => 25,
    };

    value_in_cent2(Coin2::Quarter(UsState::Alaska));

    // *** Matching with Option<T> ***
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // *** Matches Are Exhaustive - Catch-All Patterns and the _ Placeholder ***
    // - so we need to cover all possible edge case
    let dice = 9;
    match dice {
        3 => add_funny_hat(),
        7 => remove_funny_hat(),
        other => move_player(other), // `other` - covers every other possible value, the pattern is the variable we’ve chosen to name `other`
                                     // also catch all arm should be at the last as match evaluates from start to end
    };
    match dice {
        8 => println!("You win!"),
        _ => reroll(),
    };

    // *** Concise Control Flow with if let

    /*  Note: you can think of if let as syntax sugar for a match that runs code 
            when the value matches one pattern and then ignores all other values.
    */

    // Only If
    // a. using match
    let config_max: Option<u8> = Some(3u8);
    match config_max {
        Some(max) => println!("[match] The max config is : {max}"),
        _ => (), // We don’t want to do anything with the None value.
    }

    // b. using `if let`
    if let Some(max) = config_max {
        println!("[if-let] The max config is : {max}");
    }
    
    // If-Else
    // a. using match
    let coin2 = Coin2::Quarter(UsState::Alabama);
    let mut count = 0;
    match coin2 {
        Coin2::Quarter(state) => println!("[match] State Quarter from {:?}!", state),
        _ => count += 1
    }

    // b. using `if let`
    let coin2 = Coin2::Quarter(UsState::Alabama);
    if let Coin2::Quarter(state) = coin2 {
        println!("[if-let] State Quarter from {:?}!", state);
    } else {
        count += 1;
    }


    /*
    Note: When enum values have data inside them, 
        you can use match or if let to extract and use those values, 
        depending on how many cases you need to handle.
     */

}

fn route(ip_kind: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cent2(coin2: Coin2) -> u8 {
    match coin2 {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("The state variable is binded with UsState value!");
            println!("Penny for thought from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(val) => Some(val + 1), // binds value of `x` to `val`
    }
}

fn add_funny_hat() {}
fn remove_funny_hat() {}
fn move_player(num_of_space: u8) {}
fn reroll() {}
