use std::collections::HashMap;

fn main() {
    // first_examples();
    // overwriting_a_value();

    if_key_not_present();
}

fn first_examples() {
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


fn overwriting_a_value() {
    let mut scores: HashMap<String, u64> = HashMap::new();

    let key = "Key";
    let other = String::from("x");
    let x = &key[0..1];
    // scores.insert(x, 3); -> This errors! Because of explicit 'String' type 
    scores.insert(other, 3);

    println!("{}", x);
    println!("{}", key);
    // println!("{}", other);

    println!("{:?}", scores);
}

fn if_key_not_present() {
    let mut scores: HashMap<String, i64> = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);

    println("{:?}", scores);
}
