use std::collections::HashMap;

fn main() {
    // Create & Insert Into New Hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Demonstrate Ownership
    let field_name = String::from("Favorite color");
    let field_value = 2;
    scores.insert(field_name, field_value);
    // field_name and field_value are now owned by hashmap scores

    // Retrieve Value From Hashmap
    let option = scores.get(String::from("Favourite color").as_str());

    // Iterate Through Hashmap
    for (key, value) in &scores {
        println!("[{}] -> {}", key, value);
    }

    // Overwrite a Value
    scores.insert(String::from("Blue"), 20);
    println!("{:?}", scores);

    // Only Insert A Value if Key Has No Value
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores2);

    // Update Value Based On Pre-existing value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
