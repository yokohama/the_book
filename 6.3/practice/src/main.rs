fn main() {
    // 通常のmatch
    example1();

    // if let使用例
    example2();
}

fn example1() {
    let i: Option<u8> = Some(3);
    match i {
        Some(3) => println!("three"),
        _ => ()
    }
}

fn example2() {
    let i: Option<u8> = Some(3);
    // 代入式と似ているので注意
    if let Some(3) = i {
        println!("three");
    }
}
