#[allow(unused_variables)]
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    println!("Error Handling!");
    /*
     * 2 types of error in rust :-
     * --------------------------
     * 1) Non-Recoverable (panic!)
     * 2) Recoverable (Result<T, E>)
     */

    /*
     * Non-Recoverable
     */
    // 1. Calling panic to cause an error
    // panic!("Crash & Fly"); // Error

    // 2. Librery calling panic
    let vec = vec![1, 2, 3];
    // vec[99]; // Error

    /*
     * Recoverable
     */

    // 1. Main Errors
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };

    // 2. Errors inside Error (Nested)

    let greeting_file_result_2 = File::open("hello2.txt");
    let gereeting_file_2 = match greeting_file_result_2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello2.txt") {
                Ok(file) => file,
                Err(error) => panic!("Error creating the file: {:?}", error),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // 3. Error inside Error (Non-Nested approact)
    let greeting_file_3 = File::open("hello3.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello3.txt").unwrap_or_else(|err| {
                panic!("Problem creating the file: {:?}", err);
            })
        } else {
            panic!("Error opening the file: {:?}", error);
        }
    });

    // 4. Shortcuts for panic on error -

    // a) unwrap - panic with default message
    // let greeting_file_4 = File::open("hello4.txt").unwrap(); // ERROR

    // b) expect - panic with user provide message
    // let greeting_file_4 = File::open("hello4.txt").expect("hello4.txt should be included in the project!"); // ERROR

    // 5. Propogating the errors
    let username = get_username_from_file().unwrap();
    println!("username: {username}");

    // 6. Propagating errors using `?` operator
    let username2 = get_username_from_file_short().unwrap();
    println!("username2: {username2}");

    // 7. Using `?` operator on Option type
    let last_char = read_last_char_of_first_line("hello\nhi").expect("Maybe an empty string?");
    println!("last char: {last_char}");

    // 8. Using types for Validation (to avoid manual validation on every step);
    // * This will make sure the code calling will have handled the case of number being in rage *

    struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(val: i32) -> Guess {
            if val < 1 || val > 100 {
                panic!("Guess value must be between 1 and 100, got {val}");
            };

            Guess { value: val }
        }

        pub fn get(&self) -> i32 {
            self.value
        }
    }

    let guess = Guess::new(-1); // ERROR - as the validation will happed in the type and it will call panic
    // This range constraint should be documented in the public facing api
    println!("Guess: {}", guess.get());
}

fn get_username_from_file() -> Result<String, io::Error> {
    let user_name_file_result = File::open("username.txt");
    let mut user_name_file = match user_name_file_result {
        Ok(file) => file,
        Err(error) => {
            return Err(error);
        }
    };

    let mut user_name = String::new();
    match user_name_file.read_to_string(&mut user_name) {
        Ok(_) => Ok(user_name),
        Err(err) => Err(err),
    }
}

fn get_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;

    Ok(username)
}

fn read_last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
