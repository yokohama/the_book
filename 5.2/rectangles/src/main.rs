#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // selfを引数にとる、インスタンスメソッド
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // selfを引数にとる、インスタンスメソッド
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 関連関数。selfを引数にとらない、クラスメソッドのようなもの0
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rec1 = Rectangle{
        width: 30, 
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rec1.area()
    );

    println!("{:#?}", rec1);

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(50);
    println!("{:#?}", rect4);
}
