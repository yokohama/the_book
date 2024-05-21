#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    example1();
    example2();
    // アームのワイルドカード使用例
    example3();
}

fn example1() {
    let penny = Coin::Penny;
    let quarter = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(&penny));
    println!("{}", value_in_cents(&quarter));
}

fn example2() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn example3() {
    let penny = Coin::Penny;
    match penny {
        Coin::Penny => 1,
        _ => 0 
    };
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
