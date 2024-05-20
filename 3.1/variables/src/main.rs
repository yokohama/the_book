// 不変の変数への再代入エラー
// let hoge = "hoge";
// hoge = "moge";
// error : cannot assign twice to immutable variable...

// シャドーイングはエラーにならない
// let hoge = "hoge";
// let hoge = 2;

fn main() {
    // ミュータブルな変数の仕様
    example1();

    // シャドーイング
    // シャドーイングとはmutとは違う。可変なのではなく再定義。
    // シャドーイングのおかげで、型ごとに変数を用意しなくてよくなる。
    // 通常例）
    // let space_str: &str = "    ";
    // let space_num: i32 = space_str.len(); 
    // シャドーイング活用例)
    // let space = "    ";
    // let space = space.len();
    example2();
}

fn example1() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn example2() {
    let x = 5;
    let x = x + 1; // シャドーイング。元の変数を覆い隠す。

    // スコープの中だけでシャドーイング。
    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {}", x);
    }

    // 元の値には影響を与えない。
    println!("The value of x is: {}", x);
}
