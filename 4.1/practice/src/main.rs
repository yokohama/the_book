fn main() {
    // これはi32型でスタックに積み上げる例。
    // heep領域ではないので、moveは発生しない。
    let i1 = 1;
    let i2 = i1;
    println!("{}, {}", i1, i2);

    // [ スカラ型 ] => スタック積み上げ
    // あらゆる整数型。u32など。
    // 論理値型であるbool。trueとfalseという値がある。
    // あらゆる浮動小数点型、f64など。
    // 文字型であるchar。

    // スカラ型はCopyトレイトで実装されており、Dropが実装されていないので、moveではなくcopyとなる。
    // またスカラではないが、Copyトレイトを持つ型だけを含むタプル型もcpoyとなる。
    // 例えば、(i32, i32)はCopyだが、 (i32, String)は違うあらゆる整数型。u32など。

    // 以下は、Dropトレイトで実装されているString型の例。
    // move発生。shallow copyでなく、s1自体が解放される。
    // stackのコピーではなく、heepのmove。stackには新しいポインタが入る。
    let s1 = String::from("hello");
    let s2 = s1; // move
    // println!("{}, world!", s1); // s1はmoveされているのでエラー。
    println!("{}, world!", s2);

    // heepのdeepコピーはcloneでおこなう。
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3={}, s4={}", s3, s4);

    example1();
    example2();
}

fn example1() {
    let s = String::from("hoge");
    takes_ownership(s);
    // ちゃんと以下のエラーが出る。
    // Err: move occurs because `s` has type `String`, which does not implement the `Copy` trait.
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn example2() {
    let s1 = gives_ownership();
    let s2 = String::from("hoge");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hoge");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
