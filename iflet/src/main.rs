fn main() {
    let config_max: Option<u8> = None;

    match config_max {
        Some(max) => println!("The max is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The max is cnfig to be {}", max);
    }

    if let None = config_max {
        println!("Noen!");
    }
}
