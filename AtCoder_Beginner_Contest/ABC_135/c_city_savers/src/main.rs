use std::cmp::*;

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
    let n: usize = read();
    let mut a: Vec<u64> = read_vec();
    let b: Vec<u64> = read_vec();
    let mut killed_enemy: u64 = 0;

    // 後ろから処理
    for i in (0..n).rev()
    {
        // 討伐可能数
        let mut power = b[i];

        // i + 1番目の街の敵を可能な限り倒す
        let mut possible_subjugation = min(a[i + 1], power);
        killed_enemy += possible_subjugation;
        // 倒しただけ討伐可能数を減少
        power -= possible_subjugation;
        // 街の残敵数を減らす
        a[i + 1] -= possible_subjugation;

        // 討伐可能数が0であれば次へ
        if power == 0 { continue; }

        // i番目の街の敵を可能な限り倒す
        possible_subjugation = min(a[i], power);
        killed_enemy += possible_subjugation;
        // 街の残敵数を減らす
        a[i] -= possible_subjugation;
    }

    println!("{}", killed_enemy);
}
