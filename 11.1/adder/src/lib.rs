pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(x: i32) -> i32 {
    let result = x + 2;
    result
}

pub fn greeting(name: &str) -> String {
    format!("Hello! {}", name)
    //format!("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("1～100の間だけだよ。入力値: {}", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let x = Rectangle { width: 50, height: 50 };
        let y = Rectangle { width: 40, height: 30 };
        assert!(x.can_hold(&y));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let x = Rectangle { width: 50, height: 50 };
        let y = Rectangle { width: 100, height: 30 };
        assert!(!x.can_hold(&y));
    }

    #[test]
    fn it_adds_two() {
        let x = 8;
        let result = add_two(x);
        assert_eq!(result, 10);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "value is {}",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        let guess = Guess::new(500);
    }
}
