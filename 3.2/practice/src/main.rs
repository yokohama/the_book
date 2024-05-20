use std::io;

fn main() {
    // スカラ型の練習
    example1();

    // 四則演算子
    example2();

    // charの練習
    // Rustのcharはunicode
    example3();

    // タプル
    example4();

    // 配列
    // 他と同じように、mutをつけることにより、中身を変えることができる。
    example5();

    // インデックスがない場合
    example6();
}

fn example1() {
    // スカラ型の練習

    // let bin = b'h';
    // let x = 0xFF;
    // let o = 0o77;
    // let b = 0b000111;
    
    // シャドーイング
    let mut _i = 10;
    let _i = _i + 1;
    println!("{}", _i);

    // シャドーイング
    let _b = true;
    let _b = false;
    println!("{}", _b);
}

fn example2() {
    println!("{}", 1 + 3);
    println!("{}", 10 - 3);
    println!("{}", 10 * 3);
    println!("{}", 10.0 / 3.0);
    println!("{}", 10 % 3);
}

fn example3() {
    let a = 'c';
    let b = '文';
    let c = '😻';
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}

fn example4() {
    let tup1 = ("abc", 2, 'c', String::from("hoge2"));
    let tup2: (&str, u8, char) = ("abc", 2, 'c');
    let tup3 = &tup1;
    let (x, _y, _z) = tup2;
    println!("{}", x);
    println!("{}", tup1.0);
    println!("{}", tup1.1);
    println!("{}", tup1.2);
    println!("{}", tup3.3);
}

fn example5() {
    let mut array: [i32; 3] = [ 1, 2, 3 ];
    println!("{}", array[0]);

    array[0] = 5;
    println!("{}", array[0]);

    // 簡単な表記
    let array2 = [ 1, 2, 3];
    for a2 in array2 {
        println!("{}", a2);
    }
}

fn example6() {
    let a = [1, 2, 3, 4, 5, 6];
    println!("Please enter an arrray index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    // シャドーイング
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index,
        element
        );

}
