fn main() {
    example1();
    example2();

    // 3項演算子もどき
    example3();

    // ラベル付きのloop
    example4();

    // while
    example5();

    // フィボナッチ数列
    example6();
}

fn example1() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn example2() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn example3() {
    let condition = true;

    let hoge = if condition { 1 } else { 0 };

    println!("{}", hoge);
}

fn example4() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn example5() {
    let mut i = 10;

    while i > 0 {
        println!("{}", i);
        i -= 1;
    }

    println!("end");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // whileはforと比べループごとに境界値チェックをするので遅い。
    // また、プログラマによる添え字の長さミス(out of bouds)のが入る。
    while index < a.len() {
        println!("{}", a[index]);
        index += 1;
    }

    // forがお勧め
    // 添え字を使わない。
    // 境界（範囲）を意識しなくていい。
    // lengthを意識しなくていい。
    for element in a {
        println!("{}", element);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }
}

fn example6() {
    let a = [1, 2, 3, 5, 8, 13, 21];
    let mut i = 0;
    while i < a.len() {
        if i == 0 {
            println!("{}", a[0] + a[0]);
        } else {
            println!("{}", a[i] + a[i-1]);
        }
        i += 1;
    }
}
