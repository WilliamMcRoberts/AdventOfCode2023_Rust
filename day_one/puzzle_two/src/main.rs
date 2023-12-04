use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let lines = read_to_string("../aoc_day_1").expect("No File Found");

    let mut hash_map: HashMap<String, u32> = std::collections::HashMap::new();

    hash_map.insert("zero".to_string(), 0);
    hash_map.insert("one".to_string(), 1);
    hash_map.insert("two".to_string(), 2);
    hash_map.insert("three".to_string(), 3);
    hash_map.insert("four".to_string(), 4);
    hash_map.insert("five".to_string(), 5);
    hash_map.insert("six".to_string(), 6);
    hash_map.insert("seven".to_string(), 7);
    hash_map.insert("eight".to_string(), 8);
    hash_map.insert("nine".to_string(), 9);

    for i in 0..10 {
        hash_map.insert(format!("{}", i), i);
    }

    let mut total = 0;

    for line in lines.lines() {
        let mut first_index = line.len();
        let mut last_index = None;
        let mut first_value: u32 = 0;
        let mut last_value = 0;

        for (key, value) in hash_map.iter() {
            let index = if let Some(i) = line.find(key) {
                i
            } else {
                continue;
            };

            if index < first_index {
                first_index = index;
                first_value = *value;
            }

            let index = line.rfind(key);

            if index.is_some() && index > last_index {
                last_index = index;
                last_value = *value;
            }
        }

        total += (first_value * 10) + last_value;
    }

    println!("Total: {}", total);
}
