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

// 標準入力からn行を読み取り、各行を空白文字で分割し、各要素を指定の型に変換する関数
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}


fn main() {
    use std::cmp;

    let n: u32 = read();
    let r: Vec<Vec<i64>> = read_vec2(n);
    let mut profit: i64 = 1 - 10_i64.pow(9);
    let mut min: i64 = r[0][0];

    for i in 1..r.len()
    {
        profit = cmp::max(profit, r[i][0] - min);
        min = cmp::min(min, r[i][0]);
    }

    println!("{}", profit);
}
