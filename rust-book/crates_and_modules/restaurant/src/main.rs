use restaurant::{Order, take_order};

fn main() {
  println!("Welcome, Modules!");
  println!("This is an example file to show who can we consume the crate/lib we created!");

  let order1 = Order {
    order_number: 1,
    cust_name: String::from("Daniel"),
    item_name: String::from("Sushi"),
    amount: 99
  };

  take_order(order1);

  println!("The end!")

}