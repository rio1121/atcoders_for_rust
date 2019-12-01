// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let price: f64 = read();
    let no_tax: f64 = price / 1.08;

    if (no_tax.ceil() * 1.08).trunc() == price { println!("{}", no_tax.ceil()); } else { println!(":("); }
}
