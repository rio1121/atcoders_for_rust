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
    let n: u64 = read();
    let h: Vec<u64> = read_vec();
    let mut max_attempt = 0_u64;
    let mut now_attempt = 0_u64;

    if n == 0 { println!("{}", max_attempt); return; }

    for i in 1..n as usize
    {
        if h[i] <= h[i - 1] {
            now_attempt += 1;
        }
        else
        {
            max_attempt = max(now_attempt, max_attempt);
            now_attempt = 0;
        }
    }

    max_attempt = max(now_attempt, max_attempt);
    println!("{}", max_attempt);
}
