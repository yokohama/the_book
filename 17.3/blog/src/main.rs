extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("hoge");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("hoge", post.content());
}


#[test]
fn hoge() {
    let mut post = Post::new();
    post.add_text("hoge");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("hoge", post.content());
}

