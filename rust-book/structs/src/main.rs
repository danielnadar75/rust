struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

struct RectangleTuple(u32, u32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // NOTE: &self == self: &Self

    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// NOTE: new impl block is just for demonstration - this fn can be added to above block
impl Rectangle {
     /*
    Defination:
    We can define associated functions that don’t have self as their first parameter (and thus are not methods)
    because they don’t need an instance of the type to work with.
     */
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Chapter 5 - Structs!\n");

    // ***** 1. Struct *****

    // 1.1 Creating and instantiating strucs
    let user1 = User {
        active: true,
        email: String::from("dan@nad.com"),
        username: String::from("danny75"),
        sign_in_count: 1,
    };
    println!("1. Username: {}", user1.username);

    // 1.2
    // a. Returning object (move ownership) from function
    // b. the shorthand syntex (refer build_user function)
    let mut user2 = build_user(String::from("sammyboy"), String::from("sam@boy.com"));
    println!("2. Username: {}", user2.username);
    user2.username = String::from("sammyjammy");
    println!("2. Username: {}", user2.username);

    // 1.3 The spread operator (to reuse values from other object)
    // NOTE: THIS CAN MOVE DATA FOR REFERENCE TYPE AS SHOWN BELOW
    let test1 = User {
        active: true,
        email: String::from("test1@gmail.com"),
        username: String::from("test"),
        sign_in_count: 1,
    };

    let test2 = User {
        email: String::from("test2@gmail.com"),
        ..test1
    };

    // println!("test1: {}", test1.username); // ------> ERROR as test1.email is moved to test2 now
    println!("test1: {}", test2.username);

    // ***** 2. Tuple Struct *****
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // println!("Black is {black}");

    // ***** 3. Unit-Like Struct *****
    // 3.1 Example of unit-like struct instance
    let subject = AlwaysEqual;

    // ***** 4. Program to implement struct *****
    // 4.1 using tuple struct
    let rect1 = RectangleTuple(12, 4);
    let area_of_rect = area_tuple(&rect1);
    println!("The area of rectangle 1 [tuple struct] is {area_of_rect}");

    // 4.2 using normal struct
    let rect2 = Rectangle {
        width: 6,
        height: 10,
    };
    let area_of_rect = area(&rect2);
    println!("The area of rectangle 2 [normal struct] is {area_of_rect}");

    // 4.3 derive debug - to print objects to console for debugging
    println!("\n ----- Using :? and :#? for printing derived debug -------");
    println!("[normal print] rect2 is {:?}", rect2);
    println!("[pretty print] rect2 is {:#?}", rect2);

    // 4.4 dbg! macro - to print more debug information to console
    println!("\n ----- Using dbg! macro to debug -------");

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3); // IMP -> it takes ownership, hence pass by (&)reference!!!

    // 4.4 impl - to implement methods in a class/struct
    // let's take our area function inside Rectangle struct as it is a part of it
    println!("\n ----- Using impl to implemet methods on strucs -------");

    // a. method on a struct
    let area = rect3.area();
    println!("The rect3.area() is {area}");

    // b. getter on a struct
    println!("The width of rect3 is {}", rect3.width());

    // c. arguments to methods
    println!("Can rect3 hold rect2? {}", rect3.can_hold(&rect2));

    // d. associated functions
    // Note `::` is used for both "Associated Functions" & "Namespaces!"
    let square = Rectangle::square(3);
    println!("The square is {:?}", square);
}

// Showing how functions can return strucs
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area_tuple(dimension: &RectangleTuple) -> u32 {
    dimension.0 * dimension.1
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
