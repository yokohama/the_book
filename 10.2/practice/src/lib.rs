pub mod aggregator{
    pub trait Summary {
        fn summarize(&self) -> String;
        fn hoge(&self) -> String {
            String::from("default")
        }
        fn moge(&self) -> String;
        fn fuga(&self) -> String {
            self.moge()
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{} by {}, ({})",
                self.headline,
                self.author,
                self.location,
            )
        }
        fn moge(&self) -> String {
            String::from("moge")
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub retweet: bool,
        pub reply: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", 
                self.username, 
                self.content,
            )
        }
        fn moge(&self) -> String {
            String::from("moge")
        }
    }

    pub fn notify(item: &impl Summary) {
        println!("notify! {},", item.summarize());
    }

    // 実は上の`&impl Summary`は以下のシンタックスシュガーの省略形
    pub fn notify2<T: Summary>(item: &T) {
        println!("notify! {},", item.summarize());
    }
}
