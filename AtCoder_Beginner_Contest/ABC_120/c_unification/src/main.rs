// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

use std::cmp::*;

fn main() {
    let s: String = read();
    let svec: Vec<char> = s.chars().collect();

    let mut zero_count = 0_u64;
    let mut one_count = 0_u64;

    for i in 0..svec.len()
    {
        if svec[i] == '0' { zero_count += 1; } else { one_count += 1; }
    }

    let result = min(zero_count, one_count) * 2;
    println!("{}", result);
}
