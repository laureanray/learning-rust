enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    };
}

fn move_player(space: u8) {}
fn reroll() {}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penn!");
            return 1;
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
