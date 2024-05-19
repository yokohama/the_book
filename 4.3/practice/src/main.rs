// プリミティブ型
// 関数呼び出し時にスタックに積まれる
// データ=スタック
// let i = 32; // i32
// let c = 'c'; // char

// &str型(word)
// 実行時にサイズがわからない場合
// データ=ヒープ
// 参照とサイズ=スタックに保存
// let s1 = <外部のキーボード入力>;
// let word = &s1[..];

// &str型(s2)
// 文字列リテラルで、コンパイル時にサイズが決まっている。
// データ=静的領域
// 参照とサイズ=スタック
// let s2 = "hoge";

fn main() {
    let s = String::from("hello world!");
    let word = first_word(&s);
    // s.clear(); コンパイルエラー
    println!("{}", s);
    println!("{}", word);

    let x = 5;
    let y = &x;
    println!("{}, {}", x, y);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
