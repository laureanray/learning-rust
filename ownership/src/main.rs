fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let str = String::from("tite");

    &str
}
