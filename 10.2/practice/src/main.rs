use practice::aggregator::{
    Summary, 
    NewsArticle, 
    Tweet,
    notify,
    notify2,
};

fn main() {
    let news_article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };
    println!("{}", news_article.summarize());
    println!("{}", news_article.hoge());
    println!("{}", news_article.moge());

    let tweet = Tweet {
        username: String::from("hoge"),
        content: String::from("moge"),
        retweet: false,
        reply: false,
    };

    println!("{}", tweet.summarize());
    println!("{}", tweet.hoge());
    println!("{}", tweet.moge());
    notify(&tweet);
    notify2(&tweet);
}
