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

// 標準入力からn行を読み取り、各行を空白文字で分割し、各要素を指定の型に変換する関数
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

use std::cmp::*;

fn main() {
    let n: f64 = read();
    let abcde: Vec<Vec<u64>> = read_vec2(5);
    let mut minvalue = 1e15 as u64;

    for i in abcde
    {
        minvalue = min(i[0], minvalue);
    }

    let result: f64 = n / minvalue as f64 + 4.0;

    println!("{}", celi(result));
}
