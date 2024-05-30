use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //ex1();
    //ex2();

    // エラーの委譲(呼び出し元に、Result<T, E>を返し判断を委ねる。
    ex3();

    // エラーの異常のショートカット
    ex4();

    // もっとスマートに
    ex5();
}

fn ex1() {
    let file = File::open("hello.txt");

    let _file = match file {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("ファイルを作ろうとしたけど、{:?}", e);
                },
            }
        },
        Err(ref error) => {
            panic!("{:?}", error);
        }
    };
}

fn ex2() {
    // `.unwrap()は、Errが存在した場合、自動で`panic!`してくれる。
    // let file = File::open("hello.txt").unwrap();

    // `.expect()`は、`.unwrap()`に加えて任意のテキストを設定できる。
    let _file = File::open("hello.txt").expect("ファイルがあることを期待しているよ!");
}

fn ex3() -> Result<String, io::Error> {
    let f = File::open("hoge");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut data = String::new();
    match f.read_to_string(&mut data) {
        Ok(_) => Ok(data),
        Err(e) => Err(e)
    }
}

fn ex4() -> Result<String, io::Error> {
    let mut f = File::open("hoge")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn ex5() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hoge")?.read_to_string(&mut s)?;

    Ok(s)
}
