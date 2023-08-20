use traits::{
    notify, notify_muliple_traits, notify_multi_items, NewsArticle, Pair, Summary, Tweet,
};

fn main() {
    println!("Hello, Traits!");

    // 1. Defining & Implementing Trait
    let article = NewsArticle {
        headline: String::from("Mumbai & Rain <3"),
        location: String::from("Mumbai"),
        author: String::from("Daniel Nadar"),
        content: String::from("Heavy fail befals mumbai and it's a lovely green sight!"),
    };

    println!("New of the day: \n{}\n", article.summarize());

    // 2. Default Traits
    let tweet = Tweet {
        username: String::from("daniel_77"),
        content: String::from("Rust is so cool! Who else is using it?"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}\n", tweet.summarize());

    // 3. Traits as Parameters
    let article2 = NewsArticle {
        headline: String::from("Music is life!"),
        location: String::from("Frankfut, Germany"),
        author: String::from("Fecrick Chopin"),
        content: String::from("If music is the food of life, play on!"),
    };
    notify(&article2);
    notify(&tweet);
    notify_multi_items(&article, &article2);
    notify_muliple_traits(&article2);
    // clearer_trait_bounds_with_where() // lib.rs

    // 4. Return Type that Implement Traits
    // returns_summarizable(); // lib.rs

    // 5. Using Trait Bounds to Conditionally Implement Methods
    let pair = Pair::new(10, 20);
    pair.cmp_display();

    let pair2 = Pair::new(&article, &article2);
    // pair2.cmp_display(); // Error - As NewsArticle don't implement `Display` & `PartialOrd`

    // 6. Blanket Implementation
    /* Example of Blanket Implementation Used By Standard Library!
     * 
     * Info: We can conditionally implement a trait for any type that implements another trait. 
     * Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations.
     * For example, the standard library implements the ToString trait on any type that implements the Display trait. 
     * The impl block in the standard library looks similar to this code:
     *
     * impl < T: Display > ToString for T {
     * --snip--
     * }
     * 
     * Note: Because the Standard Library has the blanket implementation we can call 
     * `.to_string` on an integer, as interer type implements the Display Trait.
     */
    let s = 3.to_string();
    println!("s is {s}");

}
