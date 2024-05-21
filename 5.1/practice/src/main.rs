struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct User2 {
    // 以下、Stringではなく&strを称すると、lifetime parameterでエラー
    // 10章ライフタイムで学習予定
    // username: &str,
    // email: &str,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let mut user1 = User {
        username: String::from("hoge"),
        email: String::from("hoge@hoge.com"),
        sign_in_count: 3,
        active: true
    };

    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
    println!("{}", user1.active);

    user1.email = String::from("hoge2@hoge.com");

    let user2 = build_user(
        String::from("hoge3@hoge.com"), 
        String::from("hoge3")
    );
    println!("{}", user2.email);

    // 構造体更新記法
    let _user3 = User {
        email: String::from("hoge4@hoge.com"),
        username: String::from("hoge4"),
        ..user1 // 残り項目はuser1の値を代入
    };

    example1();
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    // 省略記法
        username, // 省略記法
        sign_in_count: 1,
        active: true
    }
}

fn example1() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 50, 0);
    let _origin = Point(0, 0, 0);
}

