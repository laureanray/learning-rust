enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        dbg!(&self);
    }
}

fn main() {
    let m = Message::Write(String::from("Test"));

    match m {
        Message::Quit => todo!(),
        Message::Move { x, y } => todo!(),
        Message::Write(&str) => {
            println!("{}", &str);
        }
        Message::ChangeColor(_, _, _) => todo!(),
    }

    m.call();
}
