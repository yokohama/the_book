use std::io;

struct Game {
    number: i32,
}

impl Game {
    pub fn start() {
        println!("さてゲームを始めよう! 1～100の数字を入力してくれ");
    }

    pub fn new(number: i32) -> Self {
        if number < 1 || number > 100 {
            panic!("number must be between 1 and 100, got {}", number);
        } else {
            Self { number }
        }
    }

    pub fn get_number(&self) -> i32 {
        self.number
    }
}

fn main() {
    Game::start();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("おいおい");
                continue;
            },
        };

        let game = Game::new(number);
        println!("{}", game.get_number());
    }
}
