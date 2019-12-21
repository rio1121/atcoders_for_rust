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

// ユークリッドの互除法
fn gcd(x: u64, y: u64) -> u64 {
    let low: u64;
    let high: u64;

    if x > y { 
        high = x;
        low = y;
    } else {
        low = x;
        high = y;
    }

    if low == 0 { high }
    else { gcd(high%low, low) }
}

use std::cmp::*;

fn main() {
    let n: usize = read();
    let a: Vec<u64> = read_vec();

    let mut accum_gcd_l = vec![0_u64; n];
    let mut accum_gcd_r = vec![0_u64; n];

    // 累積GCDの計算
    // n = 10 のとき、i = 3を削除したときの他の値全てにおけるGCDに答えは
    // lllxrrrrrr より、 a[0-2]のGCDとa[4-9]のGCDに対してGCDを取れば良い.
    // したがって、左から見た場合の累積GCD(この例で使うのはa[0]~a[2]のGCD)と、
    // 右からの累積GCD(この例で使うのはa[4]~a[9])を事前に計算する.
    for i in 1..n
    {
        accum_gcd_l[i]         = gcd(accum_gcd_l[i - 1], a[i - 1]);
        accum_gcd_r[n - i - 1] = gcd(accum_gcd_r[n - i], a[n - i]);
    }

    let mut max_lr_gcd = 1;

    // GCDの最大値を計算 iが置き換えるaのインデックス
    for i in 0..n
    {
        let gcd_lr = gcd(accum_gcd_l[i], accum_gcd_r[i]);
        max_lr_gcd = max(gcd_lr, max_lr_gcd);
    }

    println!("{}", max_lr_gcd);
}
