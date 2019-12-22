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

fn main() {
    let ab: Vec<u64> = read_vec();
    let a = ab[0];
    let b = ab[1];

    let ab_gcd = gcd(a, b);

    println!("{}", a/ab_gcd*b);
}
