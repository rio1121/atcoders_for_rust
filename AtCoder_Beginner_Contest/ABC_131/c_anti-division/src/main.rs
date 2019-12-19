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
fn gcd(low: u64, high: u64) -> u64 {
    if low == 0 { high }
    else { gcd(high%low, low) }
}

fn main() {
    let abcd: Vec<u64> = read_vec();
    let a = abcd[0];
    let b = abcd[1];
    let c = abcd[2];
    let d = abcd[3];

    // B以下の整数で、CでもDでも割り切れない数の個数
    let bn = b - b/c as u64 - b/d as u64 + b / (c/gcd(c, d)*d) as u64;
    // A未満の整数で、CでもDでも割り切れない数の個数
    let an = (a-1) - (a-1)/c as u64 - (a-1)/d as u64 + (a-1) / (c/gcd(c, d)*d) as u64;

    println!("{}", bn - an);
}
