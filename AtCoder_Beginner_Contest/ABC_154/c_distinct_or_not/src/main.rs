// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

// 標準入力から一行を読み取り、空白文字で分割し、各要素を指定の型に変換する関数
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn main() {
    let n: u64 = read();
    let mut a: Vec<u64> = read_vec();

    let before_len = a.len();
    a.sort();
    a.dedup();
    let after_len = a.len();

    if before_len != after_len { println!("NO"); return; }
    println!("YES");
}
