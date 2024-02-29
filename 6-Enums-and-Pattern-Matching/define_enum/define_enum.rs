enum IPAddress {
    V4(u32, u32, u32, u32),
    V6(String),
}

enum Message {
    Quit,
    MoVe { x: u32, y: u32, z: u32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let ipV4 = IPAddress::V4(192, 24, 23, 1);
    let ipV6 = IPAddress::V6(String::from("::1"));

    // enum usage
    let wt = Message::Write(String::from("Writing something!"));
    wt.call();

    // using Options
    let some_value: Option<String> = Some(String::from("Something"));
    let not_null_yet: Option<i32> = None;

    // checking if not_null_yet is a value now
    if not_null_yet.unwrap_or(0) == 0 {
        println!("There's nothing yet");
    }
}

fn route(ip: IPAddress) {}
