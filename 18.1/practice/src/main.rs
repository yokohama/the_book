fn main() {
    ex1();
    ex2();
    ex3();
    ex4();
}

enum Hoge {
    A { x: i32 },
    B { x: i32 },
    C { x: i32 },
}

impl Hoge {
    fn call(&self) {
        match self {
            Hoge::A { x } => {
                println!("A! {}", x);
            },
            Hoge::B { x } => {
                println!("B! {}", x);
            },
            Hoge::C { x } => {
                println!("C! {}", x);
            }
        }
    }
}

fn ex1() {
    let h = Hoge::A { x: 1 };
    h.call();
    let h = Hoge::B { x: 1 };
    h.call();
    let h = Hoge::C { x: 1 };
    h.call();
}

fn ex2() {
    let result: Option<&str> = Some("hoge");

    if let Some(r) = result {
        println!("{:?}", r);
    } else {
        println!("None");
    }

    let vec = vec![1, 2, 3];
    for (i, v) in vec.iter().enumerate() {
        println!("{}: {}", i, v);
    }
}

fn ex3() {
    let call = |x: i32, y: i32, p: i32| { 
        println!("x={} y={}", x*p, y*p) 
    };
    let call2 = |x: i32, y: i32, p: i32| { 
        println!("x={} y={}", x*p*p, y*p*p) 
    };

    struct Graph<T: Fn(i32, i32, i32)> { 
        point: i32,
        call: T 
    }
    impl<T: Fn(i32, i32, i32)> Graph<T> {
        fn new(point: i32, call: T) -> Self {
            Self { point, call }
        }
        fn exe(&self, x: i32, y: i32) {
            (self.call)(x, y, self.point);
        }
    }

    let hoge = Graph::new(8, call);
    let moge = Graph::new(8, call2);
    hoge.exe(1, 2);
    moge.exe(1, 2);
}

fn ex4() {
    enum Hoge {
        A,
        _B,
    }

    let hoge = Hoge::A;
    match hoge {
        Hoge::A => {
        },
        Hoge::_B => {
        }
    }

    let i = 1;
    match i {
        1 => {
        },
        _ => {
        }
    }

    let hoge: Option<String> = Some("hoge".to_string());
    match hoge {
        Some(s) => {
            println!("{}", s);
        },
        None => {
        }
    }

    let hoge = Some(5);
    if let Some(x) = hoge {
        println!("{}", x);
    } else {
        println!("None");
    };
}
