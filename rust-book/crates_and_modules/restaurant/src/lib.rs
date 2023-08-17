#![allow(dead_code, unused_variables)]

// This is just to let the compiler know we have moved our code to different file
mod house_of_front;
mod house_of_back;

// alias-ing
use house_of_front::hosting;

// Re-exporting -> These functions can be used by anyone using our crate!
pub use house_of_front::Order;
pub use hosting::take_order;
pub use house_of_back::home_deliver_order;

