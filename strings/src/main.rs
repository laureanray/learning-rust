fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s5 = format!("{s1}-{s2}-{s3}");
    println!("{s5}");
    let s4 = s1 + "-" + &s2 + "-" + &s3;
    println!("{s4}");

    let buhid = "ᝋᝋᝃ".to_string();

    for c in buhid.chars() {
        let unicde = c as u32;
        println!("{c} -> {:x}", unicde);
    }
}
