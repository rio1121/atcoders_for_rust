// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let mut n: i32 = read();
    let odd_count: i32 = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };

    let probability: f64 = odd_count as f64 / n as f64;

    println!("{}", probability);
}
