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
    let st: Vec<String> = read_vec();
    let ab: Vec<u64> = read_vec();
    let u: String = read();

    if st[0] == u { println!("{} {}", ab[0] - 1, ab[1]); return; }
    if st[1] == u { println!("{} {}", ab[0], ab[1] - 1); return; }
    println!("{} {}", ab[0], ab[1]);
}
