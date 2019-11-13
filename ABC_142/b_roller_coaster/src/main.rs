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

fn main() {
    let n_k: Vec<i32> = read_vec();
    let hn: Vec<i32> = read_vec();

    let mut number: i32 = 0;

    for i in 0..n_k[0] as usize
    {
        number += if hn[i] >= n_k[1] { 1 } else { 0 };
    }

    println!("{}", number);
}
