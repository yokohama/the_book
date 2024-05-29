use std::collections::HashMap;

fn main() {
    example1();

    // collectメソッドを使用したHashMapの初期化
    example2();

    // 所有権と要素の取得
    example3();

    // ループ
    example4();

    // 更新
    example5();
}

fn example1() {
    // Vec同様にキーも値も別の型は入れられない。
    let mut scores: HashMap<String, i8> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

fn example2() {
    let teams = vec!["Blue", "Yellow"];
    let initial_score = vec![10, 50];
    let _scores: HashMap<_, _> = teams.iter().zip(initial_score.iter()).collect();
}

fn example3() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // moveが発生
    map.insert(field_name, field_value);
    // コンパイルエラー
    // println!("{}", field_name);
    
    let field_name = String::from("Favorite number");
    let field_value = 2;

    let mut map = HashMap::new();
    // field_valueはCopyが発生
    map.insert(field_name, field_value);
    // 問題ない
    println!("{}", field_value);

    match map.get(&String::from("Favorite color")) {
        Some(_) => (),
        None => ()
    }
}

fn example4() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (k, v) in scores {
        println!("{}={}", k, v);
    }
}

fn example5() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
