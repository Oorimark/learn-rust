enum IPAddress {
    v4(u32, u32, u32, u32),
    v6(String),
};

enum Message {
    Quit,
    Move {x: u32, y: u32, z: u32},
    Write(String),
    ChangeColor(i32, i32, i32)
};

impl Message {
    fn call(&self) {}
};

fn main(){
    let ipv4 = IPAddress::v4(192, 24, 23, 1);
    let ipv6 = IPAddress::v6(String::from("::1"));

    // enum usage
    let wt = Message::Write(String::from("Writing something!"));
    wt.call();
}

fn route(ip: IPAddres) {}
