// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn calc(n: u64, k: u64) -> u64
{
    if n < k { return 0; }
    n / k + calc(n, k * 5)
}

fn main() {
    let n: u64 = read();

    // nが奇数の場合、末尾は0にならない.
    if n % 2 == 1 { println!("0"); return; }

    println!("{}", calc(n, 10));
}
