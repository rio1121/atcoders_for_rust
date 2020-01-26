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
    let hn: Vec<i64> = read_vec();
    let a: Vec<i64> = read_vec();

    let h = hn[0];

    let sum_a: i64 = a.iter().sum();
    if sum_a >= h { println!("Yes"); return; }

    println!("No");
}
