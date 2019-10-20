// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let number_of_slime: i32 = read();
    let mut slime_color: String = read();
    let mut vector = Vec::new();

    // 入力された文字列を配列化する
    for ch in slime_color.as_str().chars()
    {
        vector.push(ch);
    }
    // 隣接した重複を取り除くメソッド
    // ex. aaabbbccabbaa => abcaba
    vector.dedup();

    println!("{}", vector.len());
}
