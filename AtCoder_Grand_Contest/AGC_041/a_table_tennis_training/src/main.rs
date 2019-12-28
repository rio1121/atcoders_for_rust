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
    let nab: Vec<u64> = read_vec();
    let n = nab[0];
    let a = nab[1];
    let b = nab[2];

    // b - a が偶数
    if (b - a) % 2 == 0 {
        println!("{}", (b - a) / 2);
        return;
    }

    // b - a が奇数
    // Aの方が端に近い場合
    let mut even_cost: u64;
    if min(a - 1, n - b) == a - 1 {
        even_cost = a;
    } else { even_cost = n - b + 1; }
        
    println!("{}", even_cost + (b - a - 1) / 2);
}
