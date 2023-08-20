use std::fmt::{Debug, Display};

/*
 * Declarations
 */

pub trait Summary {
    // Without Default Implementation
    fn summarize_author(&self) -> String;

    // Default Implemention!
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

// Implement to all types (T)
impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Pair { x, y }
    }
}

// Implement to only types (T that implements `Display` & `PartialOrd` Traits)
impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

/*
 * Functions
 */

// `impl Trait` Syntax
pub fn notify(item: &impl Summary) {
    println!("1 new Notification! {}\n", item.summarize())
}

// `Trait Bound` Syntex
pub fn notify_multi_items<T: Summary>(item1: &T, item2: &T) {
    println!(
        "1 Multiple Notification! \n1. {}\n2. {}\n",
        item1.summarize(),
        item2.summarize()
    )
}

// Combining Multiple Traits Bounds
pub fn notify_muliple_traits(item: &(impl Summary + Display)) {
    println!("{}", format!("{}", item.summarize()));
}

// Combining Multiple Traits Bounds - alternate syntax
pub fn notify_muliple_traits_alt<T: Summary + Display>(item: &T) {
    println!("Called -> notify_muliple_traits_alt")
}

// Combining Multiple Traits Bounds Using `where`
fn clearer_trait_bounds_with_where<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

// Combining Multiple Traits Bounds Using `where` - alternate syntax
fn clearer_trait_bounds_with_where_alt<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("danny_77"),
        content: String::from("Hello rust!"),
        reply: false,
        retweet: true,
    }
}

/* --------- ERROR ---------
 * However, you can only use impl Trait if you’re returning a single type.
 * Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions around how the impl Trait syntax is implemented in the compiler. We’ll cover how to write a function with this behavior in “Using Trait Objects That Allow for Values of Different Types” on page 379.
 */

// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("..."),
//             location: String::from("..."),
//             author: String::from(".."),
//             content: String::from("..."),
//         }
//     } else {
//         Tweet {
//             username: String::from("..."),
//             content: String::from("..."),
//             reply: false,
//             retweet: false,
//         }
//     }
// }
