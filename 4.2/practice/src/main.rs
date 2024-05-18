fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
    example6();
    example7();
    // example8();
    example9();
}

// moveを無視した実装(コンパイルエラー)
fn example1() {
    // s1がmoveされるので、コンパイルエラー
    let s1 = String::from("hoge");
    let _i = calculate_length1(s1);
    // println!("{}, {}", s1, i); コンパイルエラー
}

// 面倒だが、関数側で再度戻す方法
fn example2() {
    let s1 = String::from("hoge");
    let (s2, len) = calculate_length2(s1);
    println!("{}, {}", s2, len);
}

// 参照を借用したパターン
fn example3() {
    let s1 = String::from("hoge");
    let len = calculate_length3(&s1);
    println!("{}, {}", s1, len);

}

// 可変な参照の借用
fn example4() {
    let mut s = String::from("hoge");
    change(&mut s);
    println!("{}", s);
}

// 同じスコープ内で可変な参照を複数に借用した場合はコンパイルエラー。
fn example5() {
    let mut s = String::from("hoge");
    let _s1 = &mut s;
    let _s2 = &mut s;
    // println!("{}, {}", s1, s2); ここをコメントアウトするとエラー。
}

// example5を解決したい場合。
fn example6() {
    let mut s = String::from("hoge");
    {
        let s1 = &mut s;
        println!("{}", s1);
    }
    let s2 = &mut s;
    println!("{}", s2);
}

fn example7() {
    let mut s = String::from("hoge");
    let _s1 = &s; // 不変な借用。問題なし。
    let _s2 = &s; // 不変な借用。問題なし。
    let _s3 = &mut s; // 他で不変な借用をしているのに、可変で借用はコンパイルエラー。
    // println!("{}, {}, {}", s1, s2, s3); ここをコメントアウトするとエラー。
}

// ダングリング(宙に浮いた)ポインタ
// 以下のメソッドはコンパイルエラー
// fn example8() {
//     let s = dangle();
// }

// example8の修正版
// 参照はなく値を返せば問題ない
// 関数のリターン時に所有権は呼び出し元関数にmoveされる。
fn example9() {
    let _s = no_dangle();
}

fn calculate_length1(s: String) -> usize {
    s.len()
}

fn calculate_length2(s: String) -> (String, usize) {
    // (s, s.len()) これだと最初のsでmoveが発生しているので、s.len()でエラー
    let len = s.len();
    (s, len)
}

fn calculate_length3(s: &String) -> usize {
    relend(&s); // また貸しも可能。
    s.len()
}

fn relend(some_string: &String) {
    println!("{}", some_string);
}

fn change(s: &mut String) {
    s.push_str("hoge");
}

// fn dangle() -> &String {
//     let s = String::from("hoge");
//     &s sへの参照をリターンしている
// } しかしここでヒープのsの中身は破棄される。参照ポインタだけ返すとコンパイルエラー。

fn no_dangle() -> String {
    let s = String::from("hoge");
    s
}
