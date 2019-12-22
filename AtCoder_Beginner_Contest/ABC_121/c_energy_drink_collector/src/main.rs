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
    let nm: Vec<usize> = read_vec();
    let n = nm[0];
    let m = nm[1];

    let mut shop = vec![Vec::new(); n];
    for i in 0..n
    {
        let data: Vec<u64> = read_vec();
        shop[i] = data;
    }

    // 価格の安いショップから前に出す
    shop.sort();

    let mut sum_price = 0_u64;
    let mut buyed_count = 0_u64;

    for i in 0..n
    {
        // 店iで購入する数
        let bc_tmp = min(m as u64 - buyed_count, shop[i][1]);

        // 更新
        sum_price += bc_tmp * shop[i][0];
        buyed_count += bc_tmp;

        if buyed_count == m as u64 { break; }
    }

    println!("{}", sum_price);
}
