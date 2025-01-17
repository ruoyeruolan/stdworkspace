// @Introduce  : 
// @File       : hashmaps.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/17 18:38
// @Description:  Storing Keys with Associated Values in Hash Maps

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score_ = scores.get(&team_name);
    let score = scores.get(&team_name).copied().unwrap_or(0);  // if the key does not exist, return 0
    println!("The score of the Blue team is: {}, {:?}", score, score_);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);
    scores.entry("Yellow".to_string()).or_insert(50);
    scores.entry("Blue".to_string()).or_insert(20);
    println!("{:?}", scores);

    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for world in text.split_whitespace() {
        let count = map.entry(world).or_insert(0);
        *count += 1;  // the value of count is a mutable reference
    }
    println!("{:?}", map);
}