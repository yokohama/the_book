pub mod q1 {
    use std::collections::HashMap;

    // 平均
    pub fn mean(vec: &Vec<i32>) -> i32 {
        println!("平均");

        let sum = vec.iter().sum::<i32>();
        sum / vec.len() as i32
    }

    // ソートして真ん中に来る値
    pub fn mideam(vec: &mut Vec<i32>) -> i32 {
        println!("ソートした中央値");

        // 中央の添え字を取得(繰上げ)
        let center_index = (vec.len() / 2) + 1;

        vec.sort();
        println!("{:?}", vec);

        match vec.get(center_index) {
            Some(_) => vec[center_index],
            None => {
                println!("Error");
                0
            }
        }
    }

    // 出現数の多い数字
    pub fn mode(numbers: &[i32]) -> i32 {
        println!("出現数の多い数字");

        let mut map = HashMap::new();
        for n in numbers {
            // *n : 参照外し。これをしないと、n が &i32 として map のキーになり、
            // Vec<&i32, i32> が生成され、後の `map.into_iter().collect()` で
            // Vec<(i32, i32)> を要求しているため、コンパイルエラーとなる。
            let count = map.entry(*n).or_insert(0);
            *count += 1;
        }

        let mut pairs: Vec<(i32, i32)> = map.into_iter().collect();
        // &a.1 : 値(タプルの.1)の借用。
        // `.cmp()`が、借用を必要をする仕様のため。
        pairs.sort_by(|a, b| b.1.cmp(&a.1));

        if let Some(max_pair) = pairs.first() {
            max_pair.0
        } else {
            0
        }
    }

}

pub mod q2 {
    pub fn pig_latin(input: &str) -> String {
        let vowels = vec!['a', 'i', 'u', 'e', 'o'];

        let trimmed = input.trim();

        let mut result = String::new();
        if let Some(first_char) = trimmed.chars().next() {
            if vowels.contains(&first_char) {
                result.push_str(trimmed);
                result.push_str("-hay");
            } else {
                let rest = &trimmed[1..];
                result.push_str(rest);
                result.push_str(&format!("-{}ay", first_char));
            }
        }

        result
    }
}
