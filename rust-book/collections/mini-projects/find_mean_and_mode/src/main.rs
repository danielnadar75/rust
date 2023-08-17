use std::collections::HashMap;

fn main() {
    println!("Hello, Colletion's Mini-Project!");
    println!("This is 01: Find the median and mode from the given list!\n");

    let mut list_of_nums: Vec<u8> = vec![4, 5, 4, 1, 6, 9, 5, 3, 2, 1, 4, 8, 4, 5, 4, 0, 3, 8];

    // 1. Finding Median!
    list_of_nums.sort();

    println!("Sorted List: {:?}", list_of_nums);


    let mean_index = list_of_nums.len() / 2;
    let median = list_of_nums[mean_index];
    println!("\nMedian: {}", median);

    // 2. Finding Mode (max encountered element)
    let mut map: HashMap<u8, u8> = HashMap::new();

    for num in &list_of_nums {
        let count = map.entry(*num).or_insert(0);
        *count += 1;
    }

    println!("\nMap: {:?}", map);

    let mut largest: u8 = 0; 
    for (key, val) in &map {
        if *val > largest {
            largest = *key;
        }
    }

    println!("\nMode: {largest}");
    
}
