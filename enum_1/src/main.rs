fn main() {
    enum_1_1();
    enum_1_2();
    enum_1_3();
    enum_1_4();
    enum_1_5();
    enum_1_6();
    enum_1_7();
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

struct Ipv4Addr {
    address: String,
}

struct Ipv6Addr {
    address: String,
}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrKind2 {
    V4(String),
    V6(String),
}

enum IpAddrKind3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum IpAddrKind4 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
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
        println!("{:?}", self);
    }
}

fn enum_1_1() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}

fn enum_1_2() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn enum_1_3() {
    let home = IpAddrKind2::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind2::V6(String::from("::1"));
}

fn enum_1_4() {
    let home = IpAddrKind3::V4(127, 0, 0, 1);
    let loopback = IpAddrKind3::V6(String::from("::1"));
}

fn enum_1_5() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn enum_1_6() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}

fn enum_1_7() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap();
}

fn route(ip_kind: IpAddrKind) {}