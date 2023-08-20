#[allow(dead_code, unused_variables)]
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/* This code means the type Point < f32 > will have a distance_from_origin method; other instances of Point < T > where T is not of type f32 will not have this method defined. */
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[allow(dead_code, unused_variables)]
#[derive(Debug)]
struct Cordinate<T, U> {
    x: T,
    y: U,
}

#[allow(dead_code, unused_variables)]
#[derive(Debug)]
struct Location<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Location<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Location<X2, Y2>) -> Location<X1, Y2> {
        let mixed = Location { x: self.x, y: other.y };
        mixed
    }
}

#[allow(dead_code, unused_variables)]
fn main() {
    println!("Hello, Generics!");

    // 1. Functions with generics
    let num_list = vec![2, 5, 7, 99, 103, 6, 22];
    let largest_num = find_largest(&num_list);
    println!("Largest Number: {largest_num}");

    let char_list = vec!['c', 'h', 'd', 'a', 'z'];
    let largest_char = find_largest(&char_list);
    println!("Largest Char: {largest_char}");

    // 2. Enum with generics
    let p1 = Point { x: 12, y: 4 };
    println!("P1: {:?}", p1);

    let cordinate = Cordinate { x: 100, y: 1.1 };
    println!("Cordinate: {:?}", cordinate);

    // 3. Methods with generics

    // a. With generic type (T)
    let p1_x = p1.x();
    println!("P1 X: {p1_x}");

    // b. With concrete type (f32)
    let p2: Point<f32> = Point { x: 2.2, y: 4.0 };
    let distance = p2.distance_from_origin();
    println!("P2 <distance from origin> : {}", distance);

    // c. Generics in Method Signature
    /* Note: A method that uses generic types different from its struct’s definition */
    let loc1 = Location { x: 10, y: 20.2 };
    let loc2 = Location { x: 'c', y: "hello" };
    println!("Location 1: {:?}", loc1);
    println!("Location 2: {:?}", loc2);
    
    let mixed_loc = loc1.mixup(loc2);
    println!("Mixed Location: {:?}", mixed_loc);

}

fn find_largest<T: core::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
