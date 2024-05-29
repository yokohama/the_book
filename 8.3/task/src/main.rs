use std::io;
use rand::Rng;
use std::collections::HashMap;

mod questions;

struct QuestionsSetting {
    numbers: Vec<i32>,
}

impl QuestionsSetting {
    fn new() -> Self {
        let index_num = rand::thread_rng().gen_range(1..20);
        let mut numbers: Vec<i32> = Vec::new();
        for _i in 1..index_num {
            numbers.push(rand::thread_rng().gen_range(1..300));
        }

        // 出現数の調整
        let rand_index = rand::thread_rng().gen_range(0..numbers.len());
        numbers.push(numbers[rand_index]);

        let qs = QuestionsSetting { numbers };
        let str = format!("\n初期値: {:?}\n", qs.numbers);
        println!("{}", str);

        qs
    }
}

fn main() {
    q1();
    q2();
}

fn q1() {
    let ns = QuestionsSetting::new().numbers;

    // 平均値
    println!("{}\n", questions::q1::mean(&ns));

    // 中央値
    let mut ns_clone = ns.clone();
    println!("{}\n", questions::q1::mideam(&mut ns_clone));

    // 出現率
    println!("{}\n", questions::q1::mode(&ns));
}

fn q2() {
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read line");

    println!("{}\n", questions::q2::pig_latin(&word));
}
