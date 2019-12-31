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
    let nk: Vec<usize> = read_vec();
    let n = nk[0];
    let k = nk[1];

    let mut h = vec![0_u64; n];
    for i in 0..n
    {
        h[i] = read();
    }

    // 値が低い順にソート
    h.sort();

    let mut maxmin_min = 1_000_000_000_u64;
    for i in 0..(n - k + 1)
    {
        maxmin_min = min(maxmin_min, h[i + k - 1] - h[i]);
    }

    println!("{}", maxmin_min);
}
