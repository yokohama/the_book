fn main() {
    // コンパイルが通る
    ex1();

    ex5_f();
}

fn ex1() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("{:?}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn ex2() {
    let x = 5;                         //
    let result;                        //
                                       //
    {                           //     //
        let y = 8;              //     //
        result = ex2_f(&x, &y); // b   // a
        println!("{}", result); //     //
    }                           //     //
                                       //
    // コンパイルエラー。              // 
    // ライフタイムが最小のbの中で、   // 
    // resultに代入している。          // 
    // なので、resultはbの中でしか     //
    // 使えない。                      //
    // println!("{}", result);         //
}                                      //

fn ex2_f<'a>(x: &'a i32, _y: &'a i32) -> &'a i32 {
    x
}

fn ex3_f<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

fn ex4_f<'a>(_x: i32, _y: i32) -> &'a i32 {
    &5
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn moge<'b>(&'b self) -> &'b str {
        "moge"
    }
    fn hoge<'b>(&self, str: &'b str) -> &'b str {
        "hoge"
    }
}

fn ex5_f() {
    let s = "hoge3";
    let i;
    {
        i = ImportantExcerpt { part: s };
    }

    println!("{:?}", i.part);
}
