fn test_closure() {
    let example_closure = |x| x;
    let _n = example_closure(5);
    // This would result in a compuler err
    // since closure types are inferred the first time
    // let s = example_closure(String::from("Hello"));
}

//
// fn capturing_ownership() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);
//
//     let only_borrows || println!("From closure: {:?}", list);
//
// }
