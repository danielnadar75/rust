use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    println!("Hello, crates!");

    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
