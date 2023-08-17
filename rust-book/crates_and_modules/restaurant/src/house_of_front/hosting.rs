use super::Order;

pub fn take_order(order: Order) {}

fn server_order(order_number: u32) -> Order {
  Order {
    order_number,
    cust_name: String::from("Daniel"),
    item_name: String::from("Shushi"),
    amount: 100
  }
}

fn take_payment() {}