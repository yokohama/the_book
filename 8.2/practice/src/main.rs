fn main() {
    // 思いつき練習
    example0();

    example1();

    // format!マクロ
    example2();

    // 安全なString配列の要素アクセス
    example3();
}

fn example0() {
    // 97と表示
    let mut a = String::from("a");
    for i in a.as_bytes() {
        println!("{}", i);
    }

    a.push_str("hoge");
    a.push('あ');

    // charはスカラ型なのでCopyされる。元のbは消えない。
    let b = 'b';
    a.push(b);
    println!("{}", b);

    // 227 129 130 と表示
    // この謎はこの章で解ることを期待
    let a = String::from("あ");
    for i in a.as_bytes() {
        println!("{}", i);
    }

    // 1(バイト)と表示
    let a = String::from("a");
    println!("{}", a.len());

    // 3(バイト)と表示
    let a = String::from("あ");
    println!("{}", a.len());

    let a = String::from("hogeあ");
    // 「h」は取得可能
    println!("{}", &a[0..1]);

    // 「あ」はエラー
    // なぜエラーなのかは、この章で解ることを期待
    //
    // thread 'main' panicked at src/main.rs:16:22:
    // byte index 5 is not a char boundary; 
    // it is inside 'あ' (bytes 4..7) of `hogeあ
    // println!("{}", &a[4..5]);

}

fn example1() {
    let mut data = String::from("initial contents");
    data.push_str("!");

    // 上記と同じ &str => String
    let data = "initial contents";
    let mut data = data.to_string();
    data.push_str("!");

    // 文字列の+演算子は、
    // 右辺が&str型である必要がある。
    // また、左辺の元の値(s1)はmoveされて消える。
    //
    // 以下の例では、&s2は、&String型で、&str型ではないが、
    // コンパイラが参照外し型強制で、&Stringを&strにしてくれている。
    let s1 = String::from("hoge");
    let s2 = String::from("moge");
    let _s3 = s1 + &s2;

    let a1 = String::from("a");
    let a2 = String::from("b");
    let a3 = String::from("c");
    let a4 = a1 + " " + &a2 + " " + &a3;
    println!("{}", a4);
}

fn example2() {
    let x = 2;
    let y = 4;
    let ans = 2 - 4;
    let s = format!("{} - {} = {}", x, y, ans);
    println!("{}", s);

    // format!マクロも引数の所有権を奪わない。
    println!("{},{},{}", x, y, ans);

    // 1バイト文字
    let a = String::from("hello");
    println!("{}", &a[0..1]);

    // マルチバイト文字
    let a = String::from("あい");
    // 「あ」を取得
    println!("{}", &a[0..3]);
    // 「い」を取得
    println!("{}", &a[3..6]);
    // エラー
    //println!("{}", &a[2..4]);
}

fn example3() {
    for c in "あいうえお".chars() {
        println!("{}", c);
    }
}
