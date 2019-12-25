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
    let _n: u64 = read();
    let a: Vec<u64> = read_vec();

    // 配列aの全ての値に対してGCDを取れば、その値が答え.
    // gcd(a,b,c) = gcd(a, gcd(b, c))
    // gcd(a, b) = gcd(a, b - a)
    // という性質から導ける
    let mut res: u64 = 0;
    for ai in a
    {
        res = gcd(res, ai);
    }
    println!("{}", res);
}
