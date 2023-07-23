fn test_closure() {
    let example_closure = |x| x;
    let n = example_closure(5);
    // This would result in a compuler err
    // since closure types are inferred the first time
    // let s = example_closure(String::from("Hello"));
}



