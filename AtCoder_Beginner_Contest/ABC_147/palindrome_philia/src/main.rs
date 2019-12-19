// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}



fn main() {
    let s: String = read();
    // vec化
    let s_vec: Vec<char> = s.chars().collect();
    let mut count: u32 = 0;

    for i in 0..(s_vec.len() / 2) as usize
    {
        if s_vec[i] != s_vec[s_vec.len() - 1 - i] { count += 1; }
    }

    println!("{}", count);
}
