use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    // println!("{field_name} -> {field_value}"); // error

    if let Some(skor) = scores.get("Blue") {
        println!("{skor}");
    } else {
        println!("Not found!");
    }

    // Iterating over hashmaps

    for (key, value) in &scores {
        println!("{key} -> {value}");
    }
}
