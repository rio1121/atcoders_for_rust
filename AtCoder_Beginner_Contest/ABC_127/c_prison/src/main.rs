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
    let nm: Vec<u64> = read_vec();
    let mut lmax: u64 = 0;
    let mut rmin: u64 = 100000;

    for _ in 0..nm[1]
    {
        let lr: Vec<u64> = read_vec();
        lmax = max(lmax, lr[0]);
        rmin = min(rmin, lr[1]);
    }

    if lmax > rmin { println!("0"); return; }

    let result = rmin - lmax + 1;
    println!("{}", result);
}
