// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    // define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit as i32);
    println!("{:?}", Message::Echo as u8);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
