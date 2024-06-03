fn main() {
    // ジェネリクスを使用しない普通の重複部分の関数化
    ex1();

    // ジェネリックを使わない、i32とcharのほぼ同様な処理
    ex2();

    // ジェネリックを使用して、ex2を1つの関数にまとめる
    // しかしcmpを使用しているためコンパイルエラーの確認で
    // 解説が終わるのでコメントアウト。
    //ex3();

    // structのジェネリックの使い分け
    ex4();

    // 上記、ex3()のコンパイルエラーの修正版(10.2から戻ってきた)
    ex5();

    // traitのジェネリック実装
    ex6();
}

fn ex1() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

fn ex2() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest number is {}", result);
}

//fn ex3() {
//    let number_list = vec![34, 50, 25, 100, 65];
//
//    let result = gen_largest(&number_list);
//    println!("The largest number is {}", result);
//
//    let char_list = vec!['y', 'm', 'a', 'q'];
//
//    let result = gen_largest(&char_list);
//    println!("The largest number is {}", result);
//}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//fn gen_largest<T>(list: &[T]) -> T {
//    let mut largest = list[0];
//
//    for &item in list {
//        if item < largest {
//            largest = item;
//        }
//    }
//
//    largest
//}

fn ex4() {
    struct Hoge<T> {
        item: T,
    }

    impl Hoge<i32> {
        fn print_i(&self) {
            println!("i32!");
        }
    }

    impl Hoge<&str> {
        fn print_s(&self) {
            println!("&str!");
        }
    }

    impl<T: std::fmt::Debug> Hoge<T> {
        fn print_debug_trate(&self) {
            println!("{:?}", self.item);
        }
    }

    impl<T> Hoge<T> {
        fn print_o(&self) {
            println!("Other!");
        }
    }

    let hoge = Hoge { item: 10 };
    hoge.print_i();

    let hoge = Hoge { item: "hoge" };
    // print_iは認識されない。コンパイルエラー。
    // hoge.print_i();

    // print_sは認識される。
    hoge.print_s();

    // 型がDebugトレイトを実装してる場合、print_debug_trateは認識される。
    let hoge = Hoge { item: vec![1, 2] };
    hoge.print_debug_trate();

    // print_oはなんでも認識される。
    let hoge = Hoge { item: 1 };
    hoge.print_o();
    let hoge = Hoge { item: 'c' };
    hoge.print_o();
    let hoge = Hoge { item: vec![1, 2] };
    hoge.print_o();
}

fn ex5() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = gen_largest2(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = gen_largest2(&char_list);
    println!("The largest number is {}", result);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = gen_largest3(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = gen_largest3(&char_list);
    println!("The largest number is {}", result);
}

// トレイト境界を明示的に指定するとコンパイルエラーが出なくなる。
// 大小の比較演算子を使いたいなら、PartialOrdトレイトを実装している型である保証をする必要がある。
// さらに、list[0]からlargestにmoveする実装があるため、moveではなくCopyトレイトを実装する型だけを保証してあげる。
//
// (まとめ)
// ジェネリック(T)には何が入るかわからないので、
// トレイト境界を明確にしてコンパイラに教える必要がある。
fn gen_largest2<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item < largest {
            largest = item;
        }
    }

    largest
}

// 上記のトレイト境界を、戻り値を&T（借用）にしてシンプルにしたバージョン
// 借用なので、CopyとかCloneとかのトレイト境界が不要。
fn gen_largest3<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item < largest {
            largest = item;
        }
    }

    largest
}

trait Introduction {
    fn my_name_is(&self);
    fn my_job_is(&self);
}

trait Member {
    fn start_job(&self) {
        println!("it's too busy!!");
    }
    fn take_a_break(&self) {
        println!("Refresh!!");
    }
}

impl<T: Introduction> Member for T {
    fn take_a_break(&self) { 
        println!("Refresh Refresh!!");
    }
}

struct DxEngineer {
    name: String,
    job: String,
}

impl Introduction for DxEngineer {
    fn my_name_is(&self) { 
        println!("my name is {}", self.name);
    }
    fn my_job_is(&self){ 
        println!("my job is {}", self.job);
    }
}

fn ex6() {
    let engineer = DxEngineer {
        name: String::from("saito"),
        job: String::from("marketer"),
    };

    engineer.my_name_is();
    engineer.my_job_is();
    engineer.start_job();
    engineer.take_a_break();
}
