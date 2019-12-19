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

// Vecの和を返す
fn sum_vec(v: &Vec<i64>) -> i64 {
    return v.iter().fold(0, |sum, x| sum + x);
}

fn main() {
    let n: usize = read();
    let b: Vec<i64> = read_vec();
    let mut a: Vec<i64> = vec![0; n];

    a[0] = b[0];

    for i in 1..(n - 1)
    {
        a[i] = min(b[i - 1], b[i]);
    }

    a[n - 1] = b[n - 2];

    println!("{}", sum_vec(&a));
}
