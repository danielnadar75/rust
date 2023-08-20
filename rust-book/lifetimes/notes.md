# Important Topic 

### Generic Lifetime Annotation
When we’re defining this function, we don’t know the concrete values that will be passed into this function, so we don’t know whether the if case or the else case will execute. We also don’t know the concrete lifetimes of the references that will be passed in, so we can’t look at the scopes as we did in Listings 10-17 and 10-18 to determine whether the reference we return will always be valid. The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of x and y relate to the lifetime of the return value. To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis.

### Example of generic lifetime annotation
&i32                // a reference 
&' a i32            // a reference with an explicit lifetime 
&' a mut i32        // a mutable reference with an explicit lifetime

### Read more on:
The book - Chapter 10 - Lifetimes
