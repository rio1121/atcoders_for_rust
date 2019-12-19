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
    let whxy: Vec<u64> = read_vec();
    let w = whxy[0];
    let h = whxy[1];
    let x = whxy[2];
    let y = whxy[3];

    // どこに(x, y)をとったとしても、分割した2つの部分が等しくなる直線が必ず1つは存在する.
    // x + x == w, y + y == h... xもyもそれぞれ幅高さの半分の値である場合
    println!("{} {}", (w as f64)*(h as f64)/2.0, (x+x==w&&y+y==h) as u32);
}
