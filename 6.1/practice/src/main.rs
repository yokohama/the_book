#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    example1();
}

fn example1() {
    let home = IpAddrKind::V4(192, 168, 0, 1);
    let _localback = IpAddrKind::V6(String::from("::1"));

    println!("{:#?}", home);
}

fn example2() {
    let _some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;
}
