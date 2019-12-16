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
    let n: usize = read();
    let mut d: Vec<u32> = read_vec();

    d.sort();

    // ソートした配列の中央左の値と右の値が等しい場合、整数Kは存在しない
    if d[n / 2 - 1] == d[n / 2] { println!("0"); return; }

    println!("{}", d[n / 2] - d[n / 2 - 1]);
}
