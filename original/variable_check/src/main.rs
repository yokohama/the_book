fn main() {
    // スカラ型
    // スタックに入る
    let _i = 1234;
    let _c = 'c';
    let _b = true;

    // &str型
    // 静的領域(.rdataセクションなど)に入る
    // スタックには静的領域への参照が入る
    let _str = "hogehoge";

    // String型
    // "mogemoge"は静的領域
    // fromでインスタンス化されたときにヒープに作成
    // スタックにはヒープへの参照が入る
    let _string = String::from("mogemoge");

    // タプル型
    // 1234 : スタック
    // 'c' : スタック
    // "hoge" : 静的領域(.rdataセクションなど)
    // "hello world!" : の初期値は静的領域。fromでインスタンス化されたときにヒープに作成
    // サイズやインデックスなどはコンパイル時に直接コード内に埋め込まれる
    let _tup = (1234, 'c', "hoge", String::from("hello world!"));

    // 配列型
    // 文字列は静的領域
    // サイズやインデックスなどはコンパイル時に直接コード内に埋め込まれる
    let _array: [&str; 3] = ["January", "February", "March"];
}
