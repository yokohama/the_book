use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;
use rand::Rng;
use core::fmt::Debug;

fn main() {
    let simulated_user_specified_value = rand::thread_rng().gen_range(1..11);

    generate_workout(
        simulated_user_specified_value,
    );
}

//struct Cacher<T: Fn(u32) -> u32> {
//    calculation: T,
//    value: Option<u32>,
//}

// 上記の書き方をスマートに
struct Cacher<T, U>
    where T: Fn(U) -> u32, 
          U: Eq + Hash + Clone + Debug,
{
    calculation: T,
    values: HashMap<U, u32>,
}

impl<T, U> Cacher<T, U> 
    where T: Fn(U) -> u32, 
          U: Eq + Hash + Clone + Debug,
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            values: HashMap::new(),
        }
    }

    fn result(&mut self, arg: U) -> u32 {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                println!("{:?}", arg);

                let v = (self.calculation)(arg.clone());
                self.values.insert(arg, v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32) {
    let mut expensive = Cacher::new(|num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        (num * 2) as u32
    });

    let mut str_expensive = Cacher::new(|str: &str| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        (str.len() * 2) as u32
    });


    let mut main_counter = 0;
    loop {
        if main_counter == 100 { break; }

        println!("Round {}.", main_counter);

        let rand = rand::thread_rng().gen_range(1..5);

        let mut norma = 0;
        loop {
            if rand == norma { break; }

            println!(
                "Today, do {} situps!",
                expensive.result(rand)
            );

            norma += 1;
        }

        println!("{:?}", expensive.values);

        main_counter += 1;
    }
}
