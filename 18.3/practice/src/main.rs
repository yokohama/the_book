fn main() {
    ex1();
    ex2();
    ex3();
    ex4();
    ex5();
    ex6();
    ex7();
    ex8();
}

fn ex1() {
    let x = 1;

    match x {
        1 => println!("one"),
        _ => println!("anything"),
    }
}

fn ex2() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(y) => {
            println!("{:?}", y);
        },
        _ => {
            println!("moge");
        }
    }

    println!("{:?}, {:?}", x, y);
}

fn ex3() {
    let x = 2;
    match x {
        1 | 2 => {
            println!("{}", x);
        },
        _ => {
            println!("None");
        }
    }
    match x {
        1..=5 => {
            println!("{}", x);
        },
        _ => {
            println!("None");
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}
fn ex4() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => {
            println!("On the x axis at {}", x);
        },
        Point { x: 0, y } => {
            println!("On the y axis at {}", y);
        },
        Point { x, y } => {
            println!("On neither axis: ({}, {})", x, y);
        }
    }
}

fn ex5() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];
    let sum_of_squares: i32 = points
        .into_iter()
        .map(|Point { x, y }| x * x + y * y)
        .sum();
    println!("{}", sum_of_squares);
    //println!("{}", points[0].x);
}

fn ex6() {
    let mut s = Some(String::from("hoge"));
    match s {
        Some(ref mut s) => {
            *s = String::from("moge");
        }
        _ => {}
    }

    println!("{:?}", s);
}

fn ex7() {
    let num = Some(4);
    match num {
        Some(n) if n < 5 => println!("hoge"),
        Some(n) => println!("{}", n),
        None => (),
    }
}

fn ex8() {
    let s = (1, 2, 3, 4);
    let (f, .., l) = s;
    println!("{}, {}", f, l);

    enum Message {
        Hello { id: u32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: n @ 1..=2 } => {
            println!("{}", n);
        },
        Message::Hello { id } => {
            println!("{}", id);
        },
    }

    let Message::Hello { id: x } = msg;
    println!("{}", x);
}
