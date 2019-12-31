// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

// 753数の数を再帰的に求める.
fn chk_753(n: u64, value: u64, bit753: u8, count: &mut u64) {
    if value > n { return; }
    if bit753 & 7 == 7 { *count += 1; }

    // add 7
    chk_753(n, value * 10 + 7, bit753 | 4, count);
    // add 5
    chk_753(n, value * 10 + 5, bit753 | 2, count);
    // add 3
    chk_753(n, value * 10 + 3, bit753 | 1, count);
}

fn main() {
    let n: u64 = read();
    let mut count: u64 = 0;
    chk_753(n, 0, 0, &mut count);

    println!("{}", count);
}
