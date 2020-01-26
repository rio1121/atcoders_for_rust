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

use std::cmp::*;

fn main() {
    let nk: Vec<i64> = read_vec();
    let mut h: Vec<i64> = read_vec();
    // ソート
    h.sort_by(|a, b| a.cmp(b));

    let k = nk[1];

    // k回分必殺技を使う
    for i in 0..k as usize
    {
        h.pop();
    }

    // 和を求める
    let sum_h: i64 = h.iter().sum();

    println!("{}", sum_h);
}
