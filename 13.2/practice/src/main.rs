fn main() {
    ex1();
    ex2();
    ex3();
}

fn ex1() {
    let v = vec![1, 2, 3];
    let mut v_iter = v.iter();

    let mut counter = 10;

    loop {
        if counter == 0 { break; }

        let value = v_iter.next();
        println!("{:?}", value);

        counter -= 1;
    };
}

fn ex2() {
    let v = vec![1, 2, 3];
    let v_iter = v.iter();

    // move発生
    let _total: i32 = v_iter.sum();

    // 以降、vは使えない。
    // println!("{:?}", v_iter);
}

fn ex3() {
    let v1 = vec![1, 2, 3];

    let v2 = v1.iter().map(|x| x * 2);
    println!("{:?}", v2); // 何もしていない

    // 実行させる
    let v3: Vec<_> = v2.collect();
    println!("{:?}", v3);

    // v2はmoveされているので存在しない
    // println!("{:?}", v2);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("andal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]

    )
}
