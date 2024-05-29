mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}
        fn _serve_order() {}
        fn _take_payment() {}
    }
}

mod back_of_house {
    // structの場合は、フィールドごとに、pubを選べる。
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // なぜこのimpがいるのか？直接構造体をnew出来ないのか？は以下に説明あり。
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
       }
    }

     // enumの場合は、pubをつけると全てのヴァリアントが公開される。
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // 夏(summer)にライムギパン付き朝食を注文
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // やっぱり別のパン
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // impleがないと以下の様に直接structを呼び出すことに。
    // しかし、seasonal_fruitはprivateなのでエラーとなる。
    // let hoge = back_of_house::Breakfast {
    //     toast: String::from("hoge"),
    //     seasonal_fruit: String::from("moge")
    // };

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}
