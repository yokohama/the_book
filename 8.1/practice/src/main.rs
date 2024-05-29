fn main() {
    //
    // Vecは同じ型しか保存できない！
    // 

    // 基本
    example1();

    // 要素の2つの取得方法
    example2();

    // borrow checker
    example3();

    // 参照外し(詳しくは15章)
    example4();

    //
    // 異なる型をVecに入れたい時(enumとの組合せ)
    //
    example5();
}

fn example1() {
    // v0に何を入れるかわからないので、型注釈がないとコンパイルエラー
    // let v0 = Vec::new();
    let _v0: Vec<i32> = Vec::new();

    // 初期化時に型注釈がないが、以降のコードでpushされているので、
    // コンパイラが型を推論。エラーにならない。
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    // 上記よりもスマート
    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    // vec!マクロを使用して型注釈を省略し、値も同時にいれる。
    let v3 = vec![1, 2, 3];
    for v in v3 {
        println!("{}", v);
    }
}

fn example2() {
    let v = vec![1, 2, 3, 4, 5];

    // Vecから&と[]を使用して、スカラ値(&i32型)を取得する方法
    // &i32はなくてもンパイラが推論してくれる
    let third: i32 = v[2];
    println!("The third element is {}", third);
    println!("{}", v[2]);

    // Vecからgetメソッドを使用して、Option<T>型を取得する方法
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 実行エラー(panic)
    //let does_not_exist = v[100];
    let does_not_exist = v.get(100);

    println!("{:?}", does_not_exist);
}

fn example3() {
    // borrow checkerがちゃんと監視してコンパイルエラーを出す例
    // Vecの一部を借用して、後からVecを変更
    // 
    // なぜ最初の要素の参照が、最後の要素の追加に影響を与えるのか？
    // - `let first = &v[0]` で、firstにはCopyではなく&により借用（参照）が
    // 保存される。
    // - Vecは要素をヒープ上で連続して配置する。
    // - もし`v.push(6)`により、現在のヒープ上に連続した空き容量がない場合
    // - 全要素+追加分を別のヒープ領域に移動させる。
    // - その場合、firstの参照先のヒープアドレスが空になってしまう。
    //
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);

    // 以下のコードで有効な参照があることを、borrow checkerが
    // 認識してコンパイルエラーを出す。
    // 以下のコード(有効な参照)がなければ、brrow checkerは
    // コンパイルエラーを出さない。
    // println!("{}", first);

    // Copyトレイトなら問題なし
    let mut v = vec![1, 2, 3, 4, 5];
    let first = v[0]; // 借用せずにmove
    v.push(6);

    println!("{}", first);
}

fn example4() {
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        // 読みは参照のままでOK
        println!("{}", i);

        // 書き込みは, *で参照外し
        *i += 1;
    }
}

fn example5() {
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut v: Vec<SpreadSheetCell> = Vec::new();
    v.push(SpreadSheetCell::Int(8));
    v.push(SpreadSheetCell::Float(0.8));
    v.push(SpreadSheetCell::Text(String::from("八")));

    println!("{}", v.len());

    v.pop();
    println!("{}", v.len());
}

