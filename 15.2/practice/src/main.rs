use std::ops::Deref;

fn main() {
    ex1();
    ex2();
    ex3();
    ex4();
}

fn ex1() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn ex2() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

fn ex3() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct CostomeSmartPointer {
    data: String,
}

impl Drop for CostomeSmartPointer {
    fn drop(&mut self) {
        println!("remove {}", self.data);
    }
}

fn ex4() {
    let _x = CostomeSmartPointer { data: String::from("hoge") };
    let _y = CostomeSmartPointer { data: String::from("moge") };
    println!("start");
}
