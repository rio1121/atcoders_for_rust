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

    for ch in slime_color.as_str().chars()
    {
        vector.push(ch);
    }
    vector.dedup();

    println!("{}", vector.len());
}
